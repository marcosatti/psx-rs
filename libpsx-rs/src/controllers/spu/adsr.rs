use std::cmp::{max, min};
use log::{debug, warn};
use num_traits::clamp;
use crate::State;
use crate::types::bitfield::Bitfield;
use crate::controllers::spu::voice::*;
use crate::resources::spu::voice::*;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Increase,
    Decrease,
}

pub unsafe fn handle_adsr_envelope(state: &State, voice_id: usize) {
    let play_state = &mut *get_play_state(state, voice_id);

    let calc_increment = |step: usize, shift: usize, direction: Direction, _exponential: bool| -> i16 {
        // TODO: exponential, cycle waiting
        let mut step_increment = (step << (max(0, 11 - shift as isize) as usize)) as i16;
        if direction == Direction::Decrease {
            step_increment = -step_increment;
        }

        step_increment
    };

    match play_state.adsr_mode {
        AdsrMode::Attack => {
            let (step, shift, direction, exponential) = get_adsr_attack_params(state, voice_id);
            let increment = calc_increment(step, shift, direction, exponential);
            let new_volume = play_state.adsr_current_volume as i32 + increment as i32;
            let new_volume = min(new_volume, std::i16::MAX as i32) as i16;
            play_state.adsr_current_volume = new_volume;

            if play_state.adsr_current_volume == std::i16::MAX {
                play_state.adsr_mode = AdsrMode::Decay;
            }
        },
        AdsrMode::Decay => {
            let (step, shift, direction, exponential) = get_adsr_decay_params(state, voice_id);
            let sustain_level = get_adsr_sustain_level(state, voice_id);
            let increment = calc_increment(step, shift, direction, exponential);
            let new_volume = play_state.adsr_current_volume as i32 + increment as i32;
            let new_volume = max(new_volume, sustain_level as i32) as i16;
            play_state.adsr_current_volume = new_volume;

            if play_state.adsr_current_volume == sustain_level as i16 {
                play_state.adsr_mode = AdsrMode::Sustain;
            }
        },
        AdsrMode::Sustain => {
            let (step, shift, direction, exponential) = get_adsr_sustain_params(state, voice_id);
            let increment = calc_increment(step, shift, direction, exponential);
            let new_volume = play_state.adsr_current_volume as i32 + increment as i32;
            let new_volume = clamp(new_volume, 0, std::i16::MAX as i32) as i16;
            play_state.adsr_current_volume = new_volume;

            // Note: release mode is used when key off is done.
        },
        AdsrMode::Release => {
            let (step, shift, direction, exponential) = get_adsr_release_params(state, voice_id);
            let increment = calc_increment(step, shift, direction, exponential);
            let new_volume = play_state.adsr_current_volume as i32 + increment as i32;
            let new_volume = max(new_volume, 0) as i16;
            play_state.adsr_current_volume = new_volume;
        },
    }

    // Volume should never go below 0...
    if play_state.adsr_current_volume < 0 {
        warn!("ADSR volume for voice {} went below zero ({})...", voice_id, play_state.adsr_current_volume);
    }

    play_state.adsr_current_volume = max(0, play_state.adsr_current_volume);

    let adsr_cvol = &mut *get_adsr_cvol(state, voice_id);
    adsr_cvol.write_u16(play_state.adsr_current_volume as u16);
}

unsafe fn get_adsr_attack_params(state: &State, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = &mut *get_adpcm_envelope(state, voice_id);

    let adsr_value = adsr.read_u32();

    let step = 7 - (Bitfield::new(8, 2).extract_from(adsr_value) as usize);
    let shift = Bitfield::new(10, 5).extract_from(adsr_value) as usize;
    let direction = Direction::Increase;
    let exponential = Bitfield::new(15, 1).extract_from(adsr_value) > 0;

    (step, shift, direction, exponential)
}

unsafe fn get_adsr_decay_params(state: &State, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = &mut *get_adpcm_envelope(state, voice_id);

    let adsr_value = adsr.read_u32();

    let step = 8;
    let shift = Bitfield::new(4, 4).extract_from(adsr_value) as usize;
    let direction = Direction::Decrease;
    let exponential = true;

    (step, shift, direction, exponential)
}

unsafe fn get_adsr_sustain_level(state: &State, voice_id: usize) -> usize {
    let adsr = &mut *get_adpcm_envelope(state, voice_id);
    let adsr_value = adsr.read_u32();
    ((Bitfield::new(0, 4).extract_from(adsr_value) as usize) + 1) * 0x800
}

unsafe fn get_adsr_sustain_params(state: &State, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = &mut *get_adpcm_envelope(state, voice_id);

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

unsafe fn get_adsr_release_params(state: &State, voice_id: usize) -> (usize, usize, Direction, bool) {
    let adsr = &mut *get_adpcm_envelope(state, voice_id);

    let adsr_value = adsr.read_u32();

    let step = 8;
    let shift = Bitfield::new(16, 5).extract_from(adsr_value) as usize;
    let direction = Direction::Decrease;
    let exponential = Bitfield::new(21, 1).extract_from(adsr_value) > 0;

    (step, shift, direction, exponential)
}
