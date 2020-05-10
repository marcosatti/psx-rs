#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

pub struct TransferState {
    pub current_mode: TransferMode,
    pub current_address: usize,
}

impl TransferState {
    pub fn new() -> TransferState {
        TransferState { 
            current_mode: TransferMode::Stop,
            current_address: 0,
        }
    }
}
