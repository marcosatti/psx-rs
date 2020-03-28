use num_traits::clamp;
use crate::resources::Resources;
use crate::types::bitfield::Bitfield;
use crate::types::register::b16_register::*;
use crate::types::stereo::*;
use crate::controllers::spu::voice::*;

#[derive(Copy, Clone, Debug)]
pub enum SweepMode {
    Linear,
    Exponential,
}

#[derive(Copy, Clone, Debug)]
pub enum SweepDirection {
    Increase,
    Decrease,
}

#[derive(Copy, Clone, Debug)]
pub enum SweepPhase {
    Positive,
    Negative,
}

pub fn transform_voice_adsr_volume(resources: &mut Resources, voice_id: usize, adpcm_sample: i16) -> i16 {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    (adpcm_sample as f64 * play_state.adsr_current_volume) as i16
}

pub fn transform_voice_volume(resources: &mut Resources, voice_id: usize, adpcm_sample: i16) -> Stereo {
    let vol_left = unsafe { &mut *get_voll(resources, voice_id) };
    let vol_right = unsafe { &mut *get_volr(resources, voice_id) };

    let process_sample = |vol: &mut B16Register| -> i16 {
        let vol_value = vol.read_u16();
        let volume_mode = Bitfield::new(15, 1).extract_from(vol_value);
        if volume_mode != 0 {
            let (sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode) = extract_sweep_params(vol_value);
            transform_sample_sweep(adpcm_sample, sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode)
        } else {
            let volume15 = Bitfield::new(0, 15).extract_from(vol_value);
            transform_sample_fixed(adpcm_sample, volume15)
        }
    };

    let left_sample = process_sample(vol_left);
    let right_sample = process_sample(vol_right);

    Stereo::new(left_sample, right_sample)
}

pub fn transform_main_volume(resources: &mut Resources, pcm_frame: Stereo) -> Stereo {
    let mvol_left = &mut resources.spu.main_volume_left;
    let mvol_right = &mut resources.spu.main_volume_right;

    let process_sample = |sample, mvol: &mut B16Register| -> i16 {
        let mvol_value = mvol.read_u16();
        let volume_mode = Bitfield::new(15, 1).extract_from(mvol_value);
        if volume_mode != 0 {
            let (sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode) = extract_sweep_params(mvol_value);
            transform_sample_sweep(sample, sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode)
        } else {
            let volume15 = Bitfield::new(0, 15).extract_from(mvol_value);
            transform_sample_fixed(sample, volume15)
        }
    };

    let left_sample = process_sample(pcm_frame.left, mvol_left);
    let right_sample = process_sample(pcm_frame.right, mvol_right);

    Stereo::new(left_sample, right_sample)
}

fn extract_sweep_params(sweep_raw: u16) -> (usize, usize, SweepPhase, SweepDirection, SweepMode) {
    let sweep_step = Bitfield::new(0, 2).extract_from(sweep_raw) as usize;

    let sweep_shift = Bitfield::new(2, 5).extract_from(sweep_raw) as usize;

    let sweep_phase = if Bitfield::new(12, 1).extract_from(sweep_raw) != 0 {
        SweepPhase::Negative
    } else {
        SweepPhase::Positive
    };

    let sweep_direction = if Bitfield::new(13, 1).extract_from(sweep_raw) != 0 {
        SweepDirection::Decrease
    } else {
        SweepDirection::Increase
    };
    
    let sweep_mode = if Bitfield::new(14, 1).extract_from(sweep_raw) != 0 {
        SweepMode::Exponential
    } else {
        SweepMode::Linear
    };

    (sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode)
}

fn transform_sample_fixed(sample: i16, volume15: u16) -> i16 {
    // volume15 is a 15-bit signed integer (fixed mode volume level).
    // Sign-extend it to 16-bit and double it as per docs.
    let signed_volume = (((volume15 << 1) as i16) >> 1) * 2;

    let scale_factor = clamp(signed_volume as f64 / std::i16::MAX as f64, -1.0, 1.0);

    let result = (sample as f64 * scale_factor) as i16;
    
    //debug!("sample: {}, volume15: {}, signed_volume: {}, scale_factor: {}, result: {}", sample, volume15, signed_volume, scale_factor, result);

    result
}

fn transform_sample_sweep(sample: i16, step: usize, shift: usize, phase: SweepPhase, direction: SweepDirection, mode: SweepMode) -> i16 {
    unimplemented!("sample: {}, step: {}, shift: {}, phase: {:?}, direction: {:?}, mode: {:?}", sample, step, shift, phase, direction, mode);
}
