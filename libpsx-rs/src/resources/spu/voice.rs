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
    /// Sample memory for decoding
    pub old_sample: i16,
    pub older_sample: i16,
    /// Decoded ADPCM header parameters
    pub params: AdpcmParams,
    /// Decoded raw ADPCM samples
    pub sample_buffer: Option<[i16; 28]>,
}

impl AdpcmState {
    pub fn new() -> AdpcmState {
        AdpcmState {
            old_sample: 0,
            older_sample: 0,
            params: AdpcmParams::new(),
            sample_buffer: None,
        }
    }
}

#[derive(Debug)]
pub enum AdsrMode {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug)]
pub struct PlayState {
    /// Voice (ADPCM) decoding address
    pub current_address: usize,
    /// ADPCM decoding state
    pub adpcm_state: AdpcmState,
    /// Voice sample/pitch counter
    /// Explanation (docs were a bit confusing):
    /// This is used as an interpolation counter; the SPU always plays samples at 44100 Hz, 
    /// but needs to interpolate between samples when the sample rate register is not 0x1000
    /// (44100 Hz). The upper 12+ bits are used as the ADPCM sample index, while the lower 
    /// 0-11 bits are used as an interpolation index. Notice that when the sample rate is
    /// 0x1000 (this is added to the pitch counter on every tick), the ADPCM index is always
    /// incrementing by 1, and the interpolation index by 0 (ie: perfectly in sync with 
    /// samples). When the sample rate is, say, 0x800 (22050 Hz), then sample 0 is used on 
    /// tick 0 and 1, with interpolation being used on tick 1.
    pub pitch_counter_base: usize,
    pub pitch_counter_interp: usize,
    /// Sample memory
    /// These samples are used in the interpolation process described above. As I understand
    /// it, these are the actual full ("base") samples used on each tick by the 12+ bits part
    /// of the sample counter. This is different to the ADPCM decoding sample memory, which
    /// is only related to the decoding process.
    pub old_sample: i16,
    pub older_sample: i16,
    pub oldest_sample: i16,
    /// PCM sample buffer
    /// This is filled with the output of the SPU after all processing is done.
    pub sample_buffer: Vec<Stereo>,
    /// ADSR parameters
    pub adsr_mode: AdsrMode,
    pub adsr_current_volume: i16,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            current_address: 0x1000,
            adpcm_state: AdpcmState::new(),
            pitch_counter_base: 0,
            pitch_counter_interp: 0,
            old_sample: 0,
            older_sample: 0,
            oldest_sample: 0,
            sample_buffer: Vec::new(),
            adsr_mode: AdsrMode::Attack,
            adsr_current_volume: 0,
        }
    }
}
