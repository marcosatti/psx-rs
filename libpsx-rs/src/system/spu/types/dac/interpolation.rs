#[derive(Copy, Clone, Debug)]
pub struct InterpolationState {
    /// Interpolation sample memory.
    /// These samples are only used in the interpolation process. This is different to the ADPCM
    /// decoding sample memory, which is only related to the decoding process. For example, if
    /// the decoding address suddenly jumps, the interpolation process will still be performed
    /// against the previously decoded samples.
    pub old_sample: i16,
    pub older_sample: i16,
    pub oldest_sample: i16,
}

impl InterpolationState {
    pub fn new() -> InterpolationState {
        InterpolationState {
            old_sample: 0,
            older_sample: 0,
            oldest_sample: 0,
        }
    }
}
