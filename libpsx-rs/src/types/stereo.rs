#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub(crate) struct Stereo {
    pub(crate) left: i16,
    pub(crate) right: i16,
}

impl Stereo {
    pub(crate) fn new(left: i16, right: i16) -> Stereo {
        Stereo {
            left,
            right,
        }
    }
}
