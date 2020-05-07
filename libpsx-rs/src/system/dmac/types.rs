use crate::{
    system::dmac::constants::*,
    types::{
        bitfield::Bitfield,
        memory::*,
    },
};
use parking_lot::Mutex;
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferDirection {
    FromChannel,
    ToChannel,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StepDirection {
    Forwards,
    Backwards,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyncMode {
    Continuous,
    Blocks,
    LinkedList,
}

#[derive(Debug, Copy, Clone)]
pub struct DebugState {
    pub transfer_id: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum SyncModeState {
    Undefined,
    Continuous(ContinuousState),
    Blocks(BlocksState),
    LinkedList(LinkedListState),
}

pub struct Dicr {
    pub mutex: Mutex<()>,
    pub register: B32LevelRegister,
}

impl Dicr {
    pub fn new() -> Dicr {
        Dicr {
            mutex: Mutex::new(()),
            register: B32LevelRegister::new(),
        }
    }

    pub fn read_u32(&self) -> u32 {
        self.register.read_u32()
    }

    pub fn write_u32(&self, value: u32) {
        let _lock = self.mutex.lock();
        let mut register_value = self.register.read_u32();
        register_value = Bitfield::new(0, 6).copy(register_value, value);
        register_value = Bitfield::new(15, 1).copy(register_value, value);
        register_value = Bitfield::new(16, 7).copy(register_value, value);
        register_value = Bitfield::new(23, 1).copy(register_value, value);
        register_value = Bitfield::new(24, 7).acknowledge(register_value, value);
        // Always reset the master IRQ bit - the DMAC will assert it again if it needs to.
        register_value = Bitfield::new(31, 1).insert_into(register_value, 0);
        self.register.write_u32(register_value);
    }
}

pub struct Chcr {
    pub register: B32LevelRegister,
    pub write_latch: AtomicBool,
    pub is_otc: bool,
}

impl Chcr {
    pub fn new(is_otc: bool) -> Chcr {
        let register = B32LevelRegister::new();

        register.write_u32(if is_otc {
            0x0000_0002
        } else {
            0x0000_0000
        });

        Chcr {
            register,
            write_latch: AtomicBool::new(false),
            is_otc,
        }
    }

    pub fn read_u32(&self) -> u32 {
        self.register.read_u32()
    }

    pub fn write_u32(&self, value: u32) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);

        let mut register_value = self.register.read_u32();

        if self.is_otc {
            register_value = CHCR_STARTBUSY.copy(register_value, value);
            register_value = CHCR_STARTTRIGGER.copy(register_value, value);
            register_value = CHCR_BIT30.copy(register_value, value);
        } else {
            register_value = value;
        }

        self.register.write_u32(register_value);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TransferState {
    pub started: bool,
    pub sync_mode_state: SyncModeState,
    pub debug_state: Option<DebugState>,
}

impl TransferState {
    pub fn reset() -> TransferState {
        TransferState {
            started: false,
            sync_mode_state: SyncModeState::Undefined,
            debug_state: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ContinuousState {
    pub current_address: u32,
    pub target_count: usize,
    pub current_count: usize,
}

impl ContinuousState {
    pub fn increment(&mut self, direction: StepDirection) {
        match direction {
            StepDirection::Forwards => self.current_address += DATA_SIZE,
            StepDirection::Backwards => self.current_address -= DATA_SIZE,
        }

        self.current_count += 1;
    }

    pub fn transfers_remaining(&self) -> usize {
        self.target_count - self.current_count
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlocksState {
    pub current_address: u32,
    pub current_bsize_count: usize,
    pub target_bsize_count: usize,
    pub current_bamount_count: usize,
    pub target_bamount_count: usize,
}

impl BlocksState {
    pub fn increment(&mut self, direction: StepDirection) {
        match direction {
            StepDirection::Forwards => self.current_address += DATA_SIZE,
            StepDirection::Backwards => self.current_address -= DATA_SIZE,
        }

        self.current_bsize_count += 1;
        if self.current_bsize_count == self.target_bsize_count {
            self.current_bsize_count = 0;
            self.current_bamount_count += 1;
        }
    }

    pub fn transfers_remaining(&self) -> usize {
        let target = self.target_bsize_count * self.target_bamount_count;
        let current = (self.current_bamount_count * self.target_bsize_count) + self.current_bsize_count;
        target - current
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LinkedListState {
    pub current_header_address: u32,
    pub next_header_address: u32,
    pub target_count: usize,
    pub current_count: usize,
}

impl LinkedListState {
    pub fn increment(&mut self) {
        self.current_count += 1;
    }

    pub fn transfers_remaining(&self) -> usize {
        self.target_count - self.current_count
    }
}

pub struct ControllerState {
    /// Channel transfer states.
    pub mdecin_transfer_state: TransferState,
    pub mdecout_transfer_state: TransferState,
    pub gpu_transfer_state: TransferState,
    pub cdrom_transfer_state: TransferState,
    pub spu_transfer_state: TransferState,
    pub pio_transfer_state: TransferState,
    pub otc_transfer_state: TransferState,

    /// Number of runs to cool off (not run).
    /// Intended for cases where the DMAC is holding the bus preventing the CPU from doing any work.
    pub cooloff_runs: usize,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            mdecin_transfer_state: TransferState::reset(),
            mdecout_transfer_state: TransferState::reset(),
            gpu_transfer_state: TransferState::reset(),
            cdrom_transfer_state: TransferState::reset(),
            spu_transfer_state: TransferState::reset(),
            pio_transfer_state: TransferState::reset(),
            otc_transfer_state: TransferState::reset(),
            cooloff_runs: 0,
        }
    }
}

pub struct State {
    pub dpcr: B32LevelRegister,
    pub dicr: Dicr,

    pub mdecin_madr: B32LevelRegister,
    pub mdecin_bcr: B32LevelRegister,
    pub mdecin_chcr: Chcr,

    pub mdecout_madr: B32LevelRegister,
    pub mdecout_bcr: B32LevelRegister,
    pub mdecout_chcr: Chcr,

    pub gpu_madr: B32LevelRegister,
    pub gpu_bcr: B32LevelRegister,
    pub gpu_chcr: Chcr,

    pub cdrom_madr: B32LevelRegister,
    pub cdrom_bcr: B32LevelRegister,
    pub cdrom_chcr: Chcr,

    pub spu_madr: B32LevelRegister,
    pub spu_bcr: B32LevelRegister,
    pub spu_chcr: Chcr,

    pub pio_madr: B32LevelRegister,
    pub pio_bcr: B32LevelRegister,
    pub pio_chcr: Chcr,

    pub otc_madr: B32LevelRegister,
    pub otc_bcr: B32LevelRegister,
    pub otc_chcr: Chcr,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            dpcr: B32LevelRegister::new(),
            dicr: Dicr::new(),
            mdecin_madr: B32LevelRegister::new(),
            mdecin_bcr: B32LevelRegister::new(),
            mdecin_chcr: Chcr::new(false),
            mdecout_madr: B32LevelRegister::new(),
            mdecout_bcr: B32LevelRegister::new(),
            mdecout_chcr: Chcr::new(false),
            gpu_madr: B32LevelRegister::new(),
            gpu_bcr: B32LevelRegister::new(),
            gpu_chcr: Chcr::new(false),
            cdrom_madr: B32LevelRegister::new(),
            cdrom_bcr: B32LevelRegister::new(),
            cdrom_chcr: Chcr::new(false),
            spu_madr: B32LevelRegister::new(),
            spu_bcr: B32LevelRegister::new(),
            spu_chcr: Chcr::new(false),
            pio_madr: B32LevelRegister::new(),
            pio_bcr: B32LevelRegister::new(),
            pio_chcr: Chcr::new(false),
            otc_madr: B32LevelRegister::new(),
            otc_bcr: B32LevelRegister::new(),
            otc_chcr: Chcr::new(true),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
