use crate::types::{
    exclusive_state::ExclusiveState,
    memory::*,
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum TransferDirection {
    FromChannel,
    ToChannel,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum StepDirection {
    Forwards,
    Backwards,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum SyncMode {
    Continuous(ContinuousState),
    Blocks(BlocksState),
    LinkedList(LinkedListState),
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct TransferState {
    pub(crate) enabled: bool,
    pub(crate) started: bool,
    pub(crate) direction: TransferDirection,
    pub(crate) step_direction: StepDirection,
    pub(crate) sync_mode: SyncMode,
    pub(crate) interrupt_enabled: bool,
    pub(crate) interrupted: bool,
}

impl TransferState {
    pub(crate) fn new() -> TransferState {
        TransferState {
            enabled: false,
            started: false,
            direction: TransferDirection::ToChannel,
            step_direction: StepDirection::Forwards,
            sync_mode: SyncMode::Continuous(ContinuousState::new()),
            interrupt_enabled: false,
            interrupted: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct ContinuousState {
    pub(crate) current_address: u32,
    pub(crate) target_count: usize,
    pub(crate) current_count: usize,
}

impl ContinuousState {
    pub(crate) fn new() -> ContinuousState {
        ContinuousState {
            current_address: 0,
            target_count: 0,
            current_count: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct BlocksState {
    pub(crate) current_address: u32,
    pub(crate) current_bsize_count: usize,
    pub(crate) target_bsize_count: usize,
    pub(crate) current_bamount_count: usize,
    pub(crate) target_bamount_count: usize,
}

impl BlocksState {
    pub(crate) fn new() -> BlocksState {
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
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct LinkedListState {
    pub(crate) current_header_address: u32,
    pub(crate) next_header_address: u32,
    pub(crate) target_count: usize,
    pub(crate) current_count: usize,
}

impl LinkedListState {
    pub(crate) fn new() -> LinkedListState {
        LinkedListState {
            current_header_address: 0,
            next_header_address: 0,
            target_count: 0,
            current_count: 0,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    /// Synchronization state.
    pub(crate) clock: f64,

    /// Master IRQ enable flag.
    pub(crate) master_interrupt_enabled: bool,
    pub(crate) master_interrupted: bool,

    /// Channel transfer states.
    pub(crate) mdecin_transfer_state: TransferState,
    pub(crate) mdecout_transfer_state: TransferState,
    pub(crate) gpu_transfer_state: TransferState,
    pub(crate) cdrom_transfer_state: TransferState,
    pub(crate) spu_transfer_state: TransferState,
    pub(crate) pio_transfer_state: TransferState,
    pub(crate) otc_transfer_state: TransferState,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
            master_interrupt_enabled: false,
            master_interrupted: false,
            mdecin_transfer_state: TransferState::new(),
            mdecout_transfer_state: TransferState::new(),
            gpu_transfer_state: TransferState::new(),
            cdrom_transfer_state: TransferState::new(),
            spu_transfer_state: TransferState::new(),
            pio_transfer_state: TransferState::new(),
            otc_transfer_state: TransferState::new(),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) dpcr: B32EdgeRegister,
    pub(crate) dicr: B32EdgeRegister,

    pub(crate) mdecin_madr: B32LevelRegister,
    pub(crate) mdecin_bcr: B32LevelRegister,
    pub(crate) mdecin_chcr: B32EdgeRegister,

    pub(crate) mdecout_madr: B32LevelRegister,
    pub(crate) mdecout_bcr: B32LevelRegister,
    pub(crate) mdecout_chcr: B32EdgeRegister,

    pub(crate) gpu_madr: B32LevelRegister,
    pub(crate) gpu_bcr: B32LevelRegister,
    pub(crate) gpu_chcr: B32EdgeRegister,

    pub(crate) cdrom_madr: B32LevelRegister,
    pub(crate) cdrom_bcr: B32LevelRegister,
    pub(crate) cdrom_chcr: B32EdgeRegister,

    pub(crate) spu_madr: B32LevelRegister,
    pub(crate) spu_bcr: B32LevelRegister,
    pub(crate) spu_chcr: B32EdgeRegister,

    pub(crate) pio_madr: B32LevelRegister,
    pub(crate) pio_bcr: B32LevelRegister,
    pub(crate) pio_chcr: B32EdgeRegister,

    pub(crate) otc_madr: B32LevelRegister,
    pub(crate) otc_bcr: B32LevelRegister,
    pub(crate) otc_chcr: B32EdgeRegister,

    pub(crate) controller_state: ExclusiveState<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            dpcr: B32EdgeRegister::new(),
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
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
