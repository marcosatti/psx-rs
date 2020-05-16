#[derive(Copy, Clone, Debug)]
pub struct AdpcmParams {
    pub filter: usize,
    pub shift: usize,
    pub loop_end: bool,
    pub loop_repeat: bool,
    pub loop_start: bool,
}

impl AdpcmParams {
    pub fn new() -> AdpcmParams {
        AdpcmParams {
            filter: 0,
            shift: 0,
            loop_end: false,
            loop_repeat: false,
            loop_start: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AdpcmBlockRaw {
    pub header: [u8; 2],
    pub samples: [u8; 14],
}

#[derive(Copy, Clone, Debug)]
pub struct AdpcmState {
    /// Sample memory for decoding.
    pub old_sample: i16,
    pub older_sample: i16,
    /// Decoded buffer address.
    pub decoded_address: usize,
    /// Decoded samples.
    pub sample_buffer: [i16; 28],
    /// Repeat address copy flag.
    pub copy_repeat_address: bool,
}

impl AdpcmState {
    pub fn new() -> AdpcmState {
        AdpcmState {
            old_sample: 0,
            older_sample: 0,
            decoded_address: 0,
            sample_buffer: [0; 28],
            copy_repeat_address: false,
        }
    }
}
