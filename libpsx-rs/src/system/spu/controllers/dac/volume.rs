use crate::{
    system::{
        spu::{
            controllers::dac::voice::*,
            types::*,
            constants::*,
        },
        types::State,
    },
    types::{
        stereo::*,
    },
};
use num_traits::clamp;

pub fn apply_sample_volume(state: &State, controller_state: &mut ControllerState, voice_id: usize, adpcm_sample: i16) -> Stereo {
    // The incoming ADPCM sample (mono) is volume transformed 3 times, and turned into stereo.
    let adpcm_sample = transform_voice_adsr_volume(controller_state, voice_id, adpcm_sample);
    let pcm_frame = transform_voice_volume(state, voice_id, adpcm_sample);
    transform_main_volume(state, pcm_frame)
}

fn transform_voice_adsr_volume(controller_state: &mut ControllerState, voice_id: usize, adpcm_sample: i16) -> i16 {
    let play_state = get_voice_state(controller_state, voice_id);
    let adsr_volume_normalized = (play_state.adsr_state.current_volume / std::i16::MAX) as f64;
    (adpcm_sample as f64 * adsr_volume_normalized) as i16
}

fn transform_voice_volume(state: &State, voice_id: usize, adpcm_sample: i16) -> Stereo {
    let vol_left = get_voll(state, voice_id);
    let vol_right = get_volr(state, voice_id);

    transform_sample(
        vol_left.read_u16(),
        vol_right.read_u16(),
        Stereo::new(adpcm_sample, adpcm_sample),
    )
}

fn transform_main_volume(state: &State, pcm_frame: Stereo) -> Stereo {
    let mvol_left = &state.spu.main_volume_left;
    let mvol_right = &state.spu.main_volume_right;
    
    transform_sample(
        mvol_left.read_u16(),
        mvol_right.read_u16(),
        pcm_frame,
    )
}

fn transform_sample(left_volume_value: u16, right_volume_value: u16, pcm_frame: Stereo) -> Stereo {
    let process_sample = |volume_value, sample| -> i16 {
        if VOLUME_MODE.extract_from(volume_value) > 0 {
            transform_sample_sweep(sample, extract_sweep_params(volume_value))
        } else {
            transform_sample_fixed(sample, volume_value)
        }
    };

    Stereo {
        left: process_sample(left_volume_value, pcm_frame.left),
        right: process_sample(right_volume_value, pcm_frame.right),
    }
}

fn extract_sweep_params(sweep_value: u16) -> SweepParams {
    SweepParams {
        step: SWEEP_STEP.extract_from(sweep_value) as usize,
        shift: SWEEP_SHIFT.extract_from(sweep_value) as usize,
        phase: if SWEEP_PHASE.extract_from(sweep_value) > 0 {
            SweepPhase::Negative
        } else {
            SweepPhase::Positive
        },
        direction: if SWEEP_DIRECTION.extract_from(sweep_value) > 0 {
            SweepDirection::Decrease
        } else {
            SweepDirection::Increase
        },
        mode: if SWEEP_MODE.extract_from(sweep_value) > 0 {
            SweepMode::Exponential
        } else {
            SweepMode::Linear
        },
    }
}

fn transform_sample_fixed(sample: i16, volume15: u16) -> i16 {
    // volume15 is a 15-bit signed integer (fixed mode volume level).
    // Sign-extend it to 16-bit and double it as per docs.
    let signed_volume = (((volume15 << 1) as i16) >> 1) * 2;
    let scale_factor = clamp(signed_volume as f64 / std::i16::MAX as f64, -1.0, 1.0);
    (sample as f64 * scale_factor) as i16
}

fn transform_sample_sweep(sample: i16, sweep_params: SweepParams) -> i16 {
    unimplemented!("sample: {}, sweep_params: {:?}", sample, sweep_params);
}
