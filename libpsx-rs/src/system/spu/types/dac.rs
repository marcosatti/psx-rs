mod adpcm;
mod adsr;
mod interpolation;
mod sweep;

use crate::types::stereo::*;
pub(crate) use adpcm::*;
pub(crate) use adsr::*;
pub(crate) use interpolation::*;
pub(crate) use sweep::*;

#[derive(Debug)]
pub(crate) struct VoiceState {
    /// ADPCM decoding state.
    pub(crate) adpcm_state: AdpcmState,
    /// ADSR state.
    pub(crate) adsr_state: AdsrState,
    /// Interpolation state.
    pub(crate) interpolation_state: InterpolationState,
    /// Current sample block address.
    pub(crate) current_address: usize,
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
    pub(crate) sample_counter_index: usize,
    pub(crate) sample_counter_partial: usize,
    /// PCM sample buffer.
    /// This is filled with the output of the SPU after all processing is done.
    pub(crate) sample_buffer: Vec<Stereo>,
}

impl VoiceState {
    pub(crate) fn new() -> VoiceState {
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

pub(crate) struct DacState {
    pub(crate) voice0_state: VoiceState,
    pub(crate) voice1_state: VoiceState,
    pub(crate) voice2_state: VoiceState,
    pub(crate) voice3_state: VoiceState,
    pub(crate) voice4_state: VoiceState,
    pub(crate) voice5_state: VoiceState,
    pub(crate) voice6_state: VoiceState,
    pub(crate) voice7_state: VoiceState,
    pub(crate) voice8_state: VoiceState,
    pub(crate) voice9_state: VoiceState,
    pub(crate) voice10_state: VoiceState,
    pub(crate) voice11_state: VoiceState,
    pub(crate) voice12_state: VoiceState,
    pub(crate) voice13_state: VoiceState,
    pub(crate) voice14_state: VoiceState,
    pub(crate) voice15_state: VoiceState,
    pub(crate) voice16_state: VoiceState,
    pub(crate) voice17_state: VoiceState,
    pub(crate) voice18_state: VoiceState,
    pub(crate) voice19_state: VoiceState,
    pub(crate) voice20_state: VoiceState,
    pub(crate) voice21_state: VoiceState,
    pub(crate) voice22_state: VoiceState,
    pub(crate) voice23_state: VoiceState,
}

impl DacState {
    pub(crate) fn new() -> DacState {
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
