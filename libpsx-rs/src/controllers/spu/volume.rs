use log::debug;
use num_traits::clamp;
use crate::types::bitfield::*;

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

pub fn extract_sweep_params(sweep_raw: u16) -> (usize, usize, SweepPhase, SweepDirection, SweepMode) {
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

pub fn transform_sample_fixed(sample: i16, volume15: u16) -> i16 {
    // volume15 is a 15-bit signed integer (fixed mode volume level).
    // Sign-extend it to 16-bit and double it as per docs.
    let signed_volume = (((volume15 << 1) as i16) >> 1) * 2;

    let scale_factor = clamp(signed_volume as f64 / std::i16::MAX as f64, -1.0, 1.0);

    let result = (sample as f64 * scale_factor) as i16;
    
    //debug!("sample: {}, volume15: {}, signed_volume: {}, scale_factor: {}, result: {}", sample, volume15, signed_volume, scale_factor, result);

    result
}

pub fn transform_sample_sweep(sample: i16, step: usize, shift: usize, phase: SweepPhase, direction: SweepDirection, mode: SweepMode) -> i16 {
    unimplemented!("sample: {}, step: {}, shift: {}, phase: {:?}, direction: {:?}, mode: {:?}", sample, step, shift, phase, direction, mode);
}
