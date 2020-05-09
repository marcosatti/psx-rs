#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

pub struct TransferState {
    pub current_transfer_mode: TransferMode,
    pub current_transfer_address: usize,
}

impl TransferState {
    pub fn new() -> TransferState {
        TransferState { 
            current_transfer_mode: TransferMode::Stop,
            current_transfer_address: 0,
        }
    }
}
