mod sweep;
mod adsr;
mod adpcm;
mod interpolation;

pub use sweep::*;
pub use adsr::*;
pub use adpcm::*;
pub use interpolation::*;
use crate::types::stereo::*;

#[derive(Debug)]
pub struct VoiceState {
    /// ADPCM decoding state.
    pub adpcm_state: AdpcmState,
    /// ADSR state.
    pub adsr_state: AdsrState,
    /// Interpolation state.
    pub interpolation_state: InterpolationState,
    /// Current sample block address.
    pub current_address: usize,
    /// Voice sample/pitch counter.
    /// Explanation (docs were a bit confusing):
    /// This is used as an interpolation counter; the SPU always plays samples at 44100 Hz,
    /// but needs to interpolate between samples when the sample rate register is not 0x1000
    /// (44100 Hz). The upper 12+ bits are used as the ADPCM sample index, while the lower
    /// 0-11 bits are used as an interpolation index. Notice that when the sample rate is
    /// 0x1000 (this is added to the pitch counter on every tick), the ADPCM index is always
    /// incrementing by 1, and the interpolation index by 0 (ie: perfectly in sync with
    /// samples). When the sample rate is, say, 0x800 (22050 Hz), then sample 0 is used on
    /// tick 0 and 1, with interpolation being used on tick 1.
    pub sample_counter_index: usize,
    pub sample_counter_partial: usize,
    /// PCM sample buffer.
    /// This is filled with the output of the SPU after all processing is done.
    pub sample_buffer: Vec<Stereo>,
}

impl VoiceState {
    pub fn new() -> VoiceState {
        VoiceState {
            adpcm_state: AdpcmState::new(),
            adsr_state: AdsrState::new(),
            interpolation_state: InterpolationState::new(),
            current_address: 0,
            sample_counter_index: 0,
            sample_counter_partial: 0,
            sample_buffer: Vec::new(),
        }
    }
}

pub struct DacState {
    pub voice0_state: VoiceState,
    pub voice1_state: VoiceState,
    pub voice2_state: VoiceState,
    pub voice3_state: VoiceState,
    pub voice4_state: VoiceState,
    pub voice5_state: VoiceState,
    pub voice6_state: VoiceState,
    pub voice7_state: VoiceState,
    pub voice8_state: VoiceState,
    pub voice9_state: VoiceState,
    pub voice10_state: VoiceState,
    pub voice11_state: VoiceState,
    pub voice12_state: VoiceState,
    pub voice13_state: VoiceState,
    pub voice14_state: VoiceState,
    pub voice15_state: VoiceState,
    pub voice16_state: VoiceState,
    pub voice17_state: VoiceState,
    pub voice18_state: VoiceState,
    pub voice19_state: VoiceState,
    pub voice20_state: VoiceState,
    pub voice21_state: VoiceState,
    pub voice22_state: VoiceState,
    pub voice23_state: VoiceState,
}

impl DacState {
    pub fn new() -> DacState {
        DacState {
            voice0_state: VoiceState::new(),
            voice1_state: VoiceState::new(),
            voice2_state: VoiceState::new(),
            voice3_state: VoiceState::new(),
            voice4_state: VoiceState::new(),
            voice5_state: VoiceState::new(),
            voice6_state: VoiceState::new(),
            voice7_state: VoiceState::new(),
            voice8_state: VoiceState::new(),
            voice9_state: VoiceState::new(),
            voice10_state: VoiceState::new(),
            voice11_state: VoiceState::new(),
            voice12_state: VoiceState::new(),
            voice13_state: VoiceState::new(),
            voice14_state: VoiceState::new(),
            voice15_state: VoiceState::new(),
            voice16_state: VoiceState::new(),
            voice17_state: VoiceState::new(),
            voice18_state: VoiceState::new(),
            voice19_state: VoiceState::new(),
            voice20_state: VoiceState::new(),
            voice21_state: VoiceState::new(),
            voice22_state: VoiceState::new(),
            voice23_state: VoiceState::new(),
        }
    }
}
