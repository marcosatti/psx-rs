use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::system::Resources;
use crate::system::dmac::register::*;
use crate::system::dmac::channel::*;
use crate::system::dmac::debug::*;
use crate::constants::dmac::*;
use std::sync::atomic::{AtomicBool, Ordering};
use parking_lot::Mutex;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::bitfield::Bitfield;
use crate::system::dmac::*;

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
    pub register: B32Register,
}

impl Dicr {
    pub fn new() -> Dicr {
        Dicr {
            mutex: Mutex::new(()),
            register: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Dicr {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        let mut register_value = self.register.read_u32();
        register_value = Bitfield::new(0, 6).copy(register_value, value);
        register_value = Bitfield::new(15, 1).copy(register_value, value);
        register_value = Bitfield::new(16, 7).copy(register_value, value);
        register_value = Bitfield::new(23, 1).copy(register_value, value);
        register_value = Bitfield::new(24, 7).acknowledge(register_value, value);
        // Always reset the master IRQ bit - the DMAC will assert it again if it needs to.
        register_value = Bitfield::new(31, 1).insert_into(register_value, 0);
        B8MemoryMap::write_u32(&mut self.register, offset, register_value)
    }
}

pub struct Chcr {
    pub register: B32Register,
    pub write_latch: AtomicBool,
}

impl Chcr {
    pub fn new() -> Chcr {
        Chcr { 
            register: B32Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for Chcr {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        //assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}


pub struct OtcChcr {
    pub chcr: Chcr,
}

impl OtcChcr {
    pub fn new() -> OtcChcr {
        let mut chcr = Chcr::new();
        chcr.register.write_u32(0x0000_0002);
        
        OtcChcr {
            chcr: chcr,
        }
    }
}

impl B8MemoryMap for OtcChcr {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.chcr, offset)
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        let mut register_value = self.chcr.register.read_u32();
        register_value = CHCR_STARTBUSY.copy(register_value, value);
        register_value = CHCR_STARTTRIGGER.copy(register_value, value);
        register_value = CHCR_BIT30.copy(register_value, value);
        B8MemoryMap::write_u32(&mut self.chcr, offset, register_value)
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

pub struct Dmac {
    pub dpcr: B32Register,
    pub dicr: Dicr,

    pub mdecin_madr: B32Register,
    pub mdecin_bcr: B32Register,
    pub mdecin_chcr: Chcr,
    pub mdecin_transfer_state: TransferState,

    pub mdecout_madr: B32Register,
    pub mdecout_bcr: B32Register,
    pub mdecout_chcr: Chcr,
    pub mdecout_transfer_state: TransferState,

    pub gpu_madr: B32Register,
    pub gpu_bcr: B32Register,
    pub gpu_chcr: Chcr,
    pub gpu_transfer_state: TransferState,

    pub cdrom_madr: B32Register,
    pub cdrom_bcr: B32Register,
    pub cdrom_chcr: Chcr,
    pub cdrom_transfer_state: TransferState,

    pub spu_madr: B32Register,
    pub spu_bcr: B32Register,
    pub spu_chcr: Chcr,
    pub spu_transfer_state: TransferState,

    pub pio_madr: B32Register,
    pub pio_bcr: B32Register,
    pub pio_chcr: Chcr,
    pub pio_transfer_state: TransferState,
    
    pub otc_madr: B32Register,
    pub otc_bcr: B32Register,
    pub otc_chcr: OtcChcr,
    pub otc_transfer_state: TransferState,

    /// Number of runs to cool off (not run).
    /// Intended for cases where the DMAC is holding the bus preventing the CPU from doing any work.
    pub cooloff_runs: usize,
}

impl Dmac {
    pub fn new() -> Dmac {
        Dmac {
            dpcr: B32Register::new(),
            dicr: Dicr::new(),
            mdecin_madr: B32Register::new(),
            mdecin_bcr: B32Register::new(),
            mdecin_chcr: Chcr::new(),
            mdecin_transfer_state: TransferState::reset(),
            mdecout_madr: B32Register::new(),
            mdecout_bcr: B32Register::new(),
            mdecout_chcr: Chcr::new(),
            mdecout_transfer_state: TransferState::reset(),
            gpu_madr: B32Register::new(),
            gpu_bcr: B32Register::new(),
            gpu_chcr: Chcr::new(),
            gpu_transfer_state: TransferState::reset(),
            cdrom_madr: B32Register::new(),
            cdrom_bcr: B32Register::new(),
            cdrom_chcr: Chcr::new(),
            cdrom_transfer_state: TransferState::reset(),
            spu_madr: B32Register::new(),
            spu_bcr: B32Register::new(),
            spu_chcr: Chcr::new(),
            spu_transfer_state: TransferState::reset(),
            pio_madr: B32Register::new(),
            pio_bcr: B32Register::new(),
            pio_chcr: Chcr::new(),
            pio_transfer_state: TransferState::reset(),
            otc_madr: B32Register::new(),
            otc_bcr: B32Register::new(),
            otc_chcr: OtcChcr::new(),         
            otc_transfer_state: TransferState::reset(),   
            cooloff_runs: 0,
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1F80_1080, 4, &mut resources.dmac.mdecin_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1084, 4, &mut resources.dmac.mdecin_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1088, 4, &mut resources.dmac.mdecin_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1090, 4, &mut resources.dmac.mdecout_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1094, 4, &mut resources.dmac.mdecout_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1098, 4, &mut resources.dmac.mdecout_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A0, 4, &mut resources.dmac.gpu_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A4, 4, &mut resources.dmac.gpu_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10A8, 4, &mut resources.dmac.gpu_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B0, 4, &mut resources.dmac.cdrom_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B4, 4, &mut resources.dmac.cdrom_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10B8, 4, &mut resources.dmac.cdrom_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C0, 4, &mut resources.dmac.spu_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C4, 4, &mut resources.dmac.spu_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10C8, 4, &mut resources.dmac.spu_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D0, 4, &mut resources.dmac.pio_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D4, 4, &mut resources.dmac.pio_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10D8, 4, &mut resources.dmac.pio_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E0, 4, &mut resources.dmac.otc_madr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E4, 4, &mut resources.dmac.otc_bcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10E8, 4, &mut resources.dmac.otc_chcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10F0, 4, &mut resources.dmac.dpcr as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_10F4, 4, &mut resources.dmac.dicr as *mut dyn B8MemoryMap);
}
