#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct InterpolationState {
    /// Interpolation sample memory.
    /// These samples are only used in the interpolation process. This is different to the ADPCM
    /// decoding sample memory, which is only related to the decoding process. For example, if
    /// the decoding address suddenly jumps, the interpolation process will still be performed
    /// against the previously decoded samples.
    pub(crate) old_sample: i16,
    pub(crate) older_sample: i16,
    pub(crate) oldest_sample: i16,
}

impl InterpolationState {
    pub(crate) fn new() -> InterpolationState {
        InterpolationState {
            old_sample: 0,
            older_sample: 0,
            oldest_sample: 0,
        }
    }
}
