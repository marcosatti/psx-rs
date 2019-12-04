use crate::resources::dmac::debug::*;
use crate::constants::dmac::*;

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
        (target - current)
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
