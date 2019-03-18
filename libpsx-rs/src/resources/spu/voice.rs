#[derive(Clone)]
pub struct PlayState {
    pub playing: bool,
    pub current_address: usize,
    pub old_sample: i16,
    pub older_sample: i16,

    pub current_adpcm_block_count: usize,
    pub current_adpcm_params: AdpcmParams,
    
    pub sample_buffer: Vec<i16>,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            playing: false,
            current_address: 0x1000,
            old_sample: 0,
            older_sample: 0,
            current_adpcm_block_count: 0,
            current_adpcm_params: AdpcmParams::new(),
            sample_buffer: Vec::new(),
        }
    }
}

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
