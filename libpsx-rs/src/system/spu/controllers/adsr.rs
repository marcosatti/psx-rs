use std::cmp::{max, min};
use log::warn;
use num_traits::clamp;
use crate::system::Resources;
use crate::types::bitfield::Bitfield;
use crate::controllers::spu::voice::*;
use crate::system::spu::voice::*;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Increase,
    Decrease,
}

pub fn handle_adsr_envelope(resources: &mut Resources, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };

    match play_state.adsr_mode {
        AdsrMode::Attack => {
            let (step, shift, direction, exponential) = get_adsr_attack_params(resources, voice_id);
            let increment = calculate_increment(play_state.adsr_current_volume, step, shift, direction, exponential);
            let new_volume = (play_state.adsr_current_volume + increment).min(1.0);
            play_state.adsr_current_volume = new_volume;

            if play_state.adsr_current_volume == 1.0 {
                play_state.adsr_mode = AdsrMode::Decay;
            }
        },
        AdsrMode::Decay => {
            let (step, shift, direction, exponential) = get_adsr_decay_params(resources, voice_id);
            let sustain_level = get_adsr_sustain_level(resources, voice_id) as f64 / std::i16::MAX as f64;
            let increment = calculate_increment(play_state.adsr_current_volume, step, shift, direction, exponential);
            let new_volume = (play_state.adsr_current_volume + increment).max(sustain_level);
            play_state.adsr_current_volume = new_volume;

            if play_state.adsr_current_volume == sustain_level {
                play_state.adsr_mode = AdsrMode::Sustain;
            }
        },
        AdsrMode::Sustain => {
            let (step, shift, direction, exponential) = get_adsr_sustain_params(resources, voice_id);
            let increment = calculate_increment(play_state.adsr_current_volume, step, shift, direction, exponential);
            let new_volume = clamp(play_state.adsr_current_volume + increment, 0.0, 1.0);
            play_state.adsr_current_volume = new_volume;

            // The change to release mode happens when key off is triggered.
        },
        AdsrMode::Release => {
            let (step, shift, direction, exponential) = get_adsr_release_params(resources, voice_id);
            let increment = calculate_increment(play_state.adsr_current_volume, step, shift, direction, exponential);
            let new_volume = (play_state.adsr_current_volume + increment).max(0.0);
            play_state.adsr_current_volume = new_volume;
        },
    }

    // Volume should never go below 0 or above 1...
    if play_state.adsr_current_volume < 0.0 || play_state.adsr_current_volume > 1.0 {
        warn!("ADSR volume for voice {} went below zero or above one ({})...", voice_id, play_state.adsr_current_volume);
    }

    play_state.adsr_current_volume = clamp(play_state.adsr_current_volume, 0.0, 1.0);

    let adsr_cvol = unsafe { &mut *get_adsr_cvol(resources, voice_id) };
    adsr_cvol.write_u16((play_state.adsr_current_volume * std::i16::MAX as f64) as u16);
}

fn get_adsr_attack_params(resources: &mut Resources, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = unsafe {&mut *get_adpcm_envelope(resources, voice_id) };

    let adsr_value = adsr.read_u32();

    let step = 7 - (Bitfield::new(8, 2).extract_from(adsr_value) as usize);
    let shift = Bitfield::new(10, 5).extract_from(adsr_value) as usize;
    let direction = Direction::Increase;
    let exponential = Bitfield::new(15, 1).extract_from(adsr_value) > 0;

    (step, shift, direction, exponential)
}

fn get_adsr_decay_params(resources: &mut Resources, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = unsafe { &mut *get_adpcm_envelope(resources, voice_id) };

    let adsr_value = adsr.read_u32();

    let step = 8;
    let shift = Bitfield::new(4, 4).extract_from(adsr_value) as usize;
    let direction = Direction::Decrease;
    let exponential = true;

    (step, shift, direction, exponential)
}

fn get_adsr_sustain_level(resources: &mut Resources, voice_id: usize) -> usize {
    let adsr = unsafe { &mut *get_adpcm_envelope(resources, voice_id) };
    let adsr_value = adsr.read_u32();
    min(((Bitfield::new(0, 4).extract_from(adsr_value) as usize) + 1) * 0x800, std::i16::MAX as usize)
}

fn get_adsr_sustain_params(resources: &mut Resources, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = unsafe { &mut *get_adpcm_envelope(resources, voice_id) };

    let adsr_value = adsr.read_u32();

    let mut step = Bitfield::new(22, 2).extract_from(adsr_value) as usize;
    let shift = Bitfield::new(24, 5).extract_from(adsr_value) as usize;
    let direction = if Bitfield::new(30, 1).extract_from(adsr_value) > 0 { 
        Direction::Decrease 
    } else { 
        Direction::Increase 
    };
    let exponential = Bitfield::new(31, 1).extract_from(adsr_value) > 0;

    step = match direction {
        Direction::Increase => 7 - step,
        Direction::Decrease => 8 - step, 
    };

    (step, shift, direction, exponential)
}

fn get_adsr_release_params(resources: &mut Resources, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = unsafe { &mut *get_adpcm_envelope(resources, voice_id) };

    let adsr_value = adsr.read_u32();

    let step = 8;
    let shift = Bitfield::new(16, 5).extract_from(adsr_value) as usize;
    let direction = Direction::Decrease;
    let exponential = Bitfield::new(21, 1).extract_from(adsr_value) > 0;

    (step, shift, direction, exponential)
}

fn calculate_increment(current_level: f64, step: usize, shift: usize, direction: Direction, exponential: bool) -> f64 {
    let current_level_abs = (current_level * std::i16::MAX as f64) as usize;

    let mut step_increment = (step << max(0, 11 - shift as isize) as usize) as f64;
    step_increment /= std::i16::MAX as f64;
    if direction == Direction::Decrease {
        step_increment = -step_increment;
    }

    let mut wait_cycles = 1 << max(0, shift as isize - 11) as usize;
    let mut step_increment_subfactor = 1.0;
    if exponential {
        if direction == Direction::Increase && (current_level_abs > 0x6000) {
            wait_cycles *= 4;
        } else if direction == Direction::Decrease {
            step_increment_subfactor = current_level_abs as f64 / std::i16::MAX as f64;
        }
    }

    let wait_cycles_factor = 1.0 / wait_cycles as f64;

    clamp(step_increment * step_increment_subfactor * wait_cycles_factor, -1.0, 1.0)
}
