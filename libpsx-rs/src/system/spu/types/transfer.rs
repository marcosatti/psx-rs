#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct TransferState {
    pub(crate) current_mode: TransferMode,
    pub(crate) current_address: usize,
}

impl TransferState {
    pub(crate) fn new() -> TransferState {
        TransferState {
            current_mode: TransferMode::Stop,
            current_address: 0,
        }
    }
}
