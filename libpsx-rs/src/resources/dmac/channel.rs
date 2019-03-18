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
pub enum SyncModeState {
    Undefined,
    Continuous(ContinuousState),
    Blocks(BlocksState),
    LinkedList(LinkedListState),
}

#[derive(Debug, Copy, Clone)]
pub struct ContinuousState {
    pub current_address: u32,
    pub target_count: usize,
    pub current_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct BlocksState {
    pub current_address: u32,
    pub current_bsize_count: usize,
    pub target_bsize_count: usize,
    pub current_bamount_count: usize,
    pub target_bamount_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct LinkedListState {
    pub current_address: u32,
    pub next_address: u32,
    pub target_count: usize,
    pub current_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct DebugState {
    pub transfer_id: usize,
}
