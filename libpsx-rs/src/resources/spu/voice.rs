use crate::types::stereo::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct AdpcmState {
    pub old_sample: i16,
    pub older_sample: i16,
    pub block_count: usize,
    pub params: AdpcmParams,
    pub sample_buffer: Vec<i16>,
}

impl AdpcmState {
    pub fn new() -> AdpcmState {
        AdpcmState {
            old_sample: 0,
            older_sample: 0,
            block_count: 0,
            params: AdpcmParams::new(),
            sample_buffer: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct PlayState {
    pub playing: bool,
    pub current_address: usize,
    pub adpcm_state: AdpcmState, 
    pub sample_buffer: Vec<Stereo>,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            playing: false,
            current_address: 0x1000,
            adpcm_state: AdpcmState::new(),
            sample_buffer: Vec::new(),
        }
    }
}
