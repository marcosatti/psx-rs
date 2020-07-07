#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug)]
pub(crate) struct AdpcmParams {
    pub(crate) filter: usize,
    pub(crate) shift: usize,
    pub(crate) loop_end: bool,
    pub(crate) loop_repeat: bool,
    pub(crate) loop_start: bool,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct AdpcmBlockRaw {
    pub(crate) header: [u8; 2],
    pub(crate) samples: [u8; 14],
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct AdpcmState {
    /// Sample memory for decoding.
    pub(crate) old_sample: i16,
    pub(crate) older_sample: i16,
    /// Decoded buffer address.
    pub(crate) decoded_address: usize,
    /// Decoded samples.
    pub(crate) sample_buffer: [i16; 28],
    /// Repeat address copy flag.
    pub(crate) copy_repeat_address: bool,
}

impl AdpcmState {
    pub(crate) fn new() -> AdpcmState {
        AdpcmState {
            old_sample: 0,
            older_sample: 0,
            decoded_address: 0,
            sample_buffer: [0; 28],
            copy_repeat_address: false,
        }
    }
}
