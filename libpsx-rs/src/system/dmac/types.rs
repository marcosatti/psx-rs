use crate::types::memory::*;
use enum_as_inner::EnumAsInner;
use parking_lot::Mutex;

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

#[derive(Debug, Copy, Clone, EnumAsInner)]
pub enum SyncMode {
    Undefined,
    Continuous(ContinuousState),
    Blocks(BlocksState),
    LinkedList(LinkedListState),
}

#[derive(Debug, Copy, Clone)]
pub struct TransferState {
    pub started: bool,
    pub direction: TransferDirection,
    pub step_direction: StepDirection,
    pub sync_mode: SyncMode,
    pub interrupt_enabled: bool,
    pub interrupted: bool,
}

impl TransferState {
    pub fn new() -> TransferState {
        TransferState {
            started: false,
            direction: TransferDirection::ToChannel,
            step_direction: StepDirection::Forwards,
            sync_mode: SyncMode::Undefined,
            interrupt_enabled: false,
            interrupted: false,
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
    pub fn new() -> ContinuousState {
        ContinuousState {
            current_address: 0,
            target_count: 0,
            current_count: 0,
        }
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
    pub fn new() -> BlocksState {
        BlocksState {
            current_address: 0,
            current_bsize_count: 0,
            target_bsize_count: 0,
            current_bamount_count: 0,
            target_bamount_count: 0,
        }
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
    pub fn new() -> LinkedListState {
        LinkedListState {
            current_header_address: 0,
            next_header_address: 0,
            target_count: 0,
            current_count: 0,
        }
    }
}

pub struct ControllerState {
    /// Master IRQ enable flag.
    pub master_interrupt_enabled: bool,
    pub master_interrupted: bool,

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
            master_interrupt_enabled: false,
            master_interrupted: false,
            mdecin_transfer_state: TransferState::new(),
            mdecout_transfer_state: TransferState::new(),
            gpu_transfer_state: TransferState::new(),
            cdrom_transfer_state: TransferState::new(),
            spu_transfer_state: TransferState::new(),
            pio_transfer_state: TransferState::new(),
            otc_transfer_state: TransferState::new(),
            cooloff_runs: 0,
        }
    }
}

pub struct State {
    pub dpcr: B32LevelRegister,
    pub dicr: B32EdgeRegister,

    pub mdecin_madr: B32LevelRegister,
    pub mdecin_bcr: B32LevelRegister,
    pub mdecin_chcr: B32EdgeRegister,

    pub mdecout_madr: B32LevelRegister,
    pub mdecout_bcr: B32LevelRegister,
    pub mdecout_chcr: B32EdgeRegister,

    pub gpu_madr: B32LevelRegister,
    pub gpu_bcr: B32LevelRegister,
    pub gpu_chcr: B32EdgeRegister,

    pub cdrom_madr: B32LevelRegister,
    pub cdrom_bcr: B32LevelRegister,
    pub cdrom_chcr: B32EdgeRegister,

    pub spu_madr: B32LevelRegister,
    pub spu_bcr: B32LevelRegister,
    pub spu_chcr: B32EdgeRegister,

    pub pio_madr: B32LevelRegister,
    pub pio_bcr: B32LevelRegister,
    pub pio_chcr: B32EdgeRegister,

    pub otc_madr: B32LevelRegister,
    pub otc_bcr: B32LevelRegister,
    pub otc_chcr: B32EdgeRegister,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            dpcr: B32LevelRegister::new(),
            dicr: B32EdgeRegister::new(),
            mdecin_madr: B32LevelRegister::new(),
            mdecin_bcr: B32LevelRegister::new(),
            mdecin_chcr: B32EdgeRegister::new(),
            mdecout_madr: B32LevelRegister::new(),
            mdecout_bcr: B32LevelRegister::new(),
            mdecout_chcr: B32EdgeRegister::new(),
            gpu_madr: B32LevelRegister::new(),
            gpu_bcr: B32LevelRegister::new(),
            gpu_chcr: B32EdgeRegister::new(),
            cdrom_madr: B32LevelRegister::new(),
            cdrom_bcr: B32LevelRegister::new(),
            cdrom_chcr: B32EdgeRegister::new(),
            spu_madr: B32LevelRegister::new(),
            spu_bcr: B32LevelRegister::new(),
            spu_chcr: B32EdgeRegister::new(),
            pio_madr: B32LevelRegister::new(),
            pio_bcr: B32LevelRegister::new(),
            pio_chcr: B32EdgeRegister::new(),
            otc_madr: B32LevelRegister::new(),
            otc_bcr: B32LevelRegister::new(),
            otc_chcr: B32EdgeRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
