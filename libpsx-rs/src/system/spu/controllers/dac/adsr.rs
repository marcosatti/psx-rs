use crate::{
    system::{
        spu::{
            controllers::dac::voice::*,
            types::*,
            constants::*,
        },
        types::State,
    },
};
use num_traits::clamp;
use std::cmp::{
    max,
    min,
};

pub fn handle_adsr_envelope(state: &State, controller_state: &mut ControllerState, voice_id: usize) {
    let play_state = get_voice_state(controller_state, voice_id);

    // Don't process anything if waiting.
    if play_state.adsr_state.wait_cycles > 0 {
        play_state.adsr_state.wait_cycles -= 1;
        return;
    }

    // Apply the next waiting ADSR volume and write to register.
    play_state.adsr_state.current_volume = play_state.adsr_state.next_volume;
    get_cvol(state, voice_id).write_u16(play_state.adsr_state.current_volume as u16);

    // Calculate the next ADSR volume & wait cycles.
    let adsr_value = get_adsr(state, voice_id).read_u32();
    let params = extract_phase_params(adsr_value, play_state.adsr_state.phase);
    let sustain_level = extract_adsr_sustain_level(adsr_value) as isize;
    let (delta_volume, wait_cycles) = calculate_envelope_delta(params, play_state.adsr_state.current_volume);
    let mut next_volume = play_state.adsr_state.current_volume as isize + delta_volume as isize;

    match play_state.adsr_state.phase {
        AdsrPhase::Attack => {
            next_volume = min(next_volume, std::i16::MAX as isize);
            if next_volume == std::i16::MAX as isize {
                play_state.adsr_state.phase = AdsrPhase::Decay;
            }
        },
        AdsrPhase::Decay => {
            next_volume = max(next_volume, sustain_level);
            if next_volume == sustain_level {
                play_state.adsr_state.phase = AdsrPhase::Sustain;
            }
        },
        AdsrPhase::Sustain => {
            next_volume = clamp(next_volume, 0, std::i16::MAX as isize);
            // The change to release phase happens when key off is triggered.
        },
        AdsrPhase::Release => {
            next_volume = max(next_volume, 0);
            // Stays in release phase forever until key on happens (back to attack).
        },
    }

    play_state.adsr_state.next_volume = next_volume as i16;
    play_state.adsr_state.wait_cycles = wait_cycles;
}

fn extract_adsr_sustain_level(adsr_value: u32) -> i16 {
    let raw_level = ADSR_SUSTAIN_LEVEL.extract_from(adsr_value) as usize;
    min((raw_level + 1) * 0x800, std::i16::MAX as usize) as i16
}

fn extract_phase_params(adsr_value: u32, phase: AdsrPhase) -> AdsrPhaseParams {
    match phase {
        AdsrPhase::Attack => AdsrPhaseParams {
            step: ADSR_ATTACK_STEP.extract_from(adsr_value) as usize,
            shift: ADSR_ATTACK_SHIFT.extract_from(adsr_value) as usize,
            direction: AdsrDirection::Increase,
            mode: if ADSR_ATTACK_MODE.extract_from(adsr_value) > 0 { 
                AdsrMode::Exponential 
            } else {
                AdsrMode::Linear
            },
        },
        AdsrPhase::Decay => AdsrPhaseParams {
            step: 0,
            shift: ADSR_DECAY_SHIFT.extract_from(adsr_value) as usize,
            direction: AdsrDirection::Decrease,
            mode: AdsrMode::Exponential,
        },
        AdsrPhase::Sustain => AdsrPhaseParams {
            step: ADSR_SUSTAIN_STEP.extract_from(adsr_value) as usize,
            shift: ADSR_SUSTAIN_SHIFT.extract_from(adsr_value) as usize,
            direction: if ADSR_SUSTAIN_DIRECTION.extract_from(adsr_value) > 0 {
                AdsrDirection::Decrease
            } else {
                AdsrDirection::Increase
            },
            mode: if ADSR_SUSTAIN_MODE.extract_from(adsr_value) > 0 { 
                AdsrMode::Exponential 
            } else {
                AdsrMode::Linear
            },
        },
        AdsrPhase::Release => AdsrPhaseParams {
            step: 0,
            shift: ADSR_RELEASE_SHIFT.extract_from(adsr_value) as usize,
            direction: AdsrDirection::Decrease,
            mode: if ADSR_RELEASE_MODE.extract_from(adsr_value) > 0 {
                AdsrMode::Exponential
            } else {
                AdsrMode::Linear
            },
        },
    }
}

fn calculate_envelope_delta(params: AdsrPhaseParams, current_level: i16) -> (i16, usize) {
    let mut wait_cycles = 1 << (max(0, params.shift as isize - 11) as usize);
    
    let base_step = match params.direction {
        AdsrDirection::Increase => 7 - (params.step as isize),
        AdsrDirection::Decrease => -8 + (params.step as isize),
    };

    let mut step = base_step << (max(0, 11 - params.shift as isize) as usize);

    if params.mode == AdsrMode::Exponential {
        if params.direction == AdsrDirection::Increase && (current_level > 0x6000) {
            wait_cycles *= 4;
        } else if params.direction == AdsrDirection::Decrease {
            step = step * (current_level as isize) / 0x8000;
        }
    }

    step = clamp(step, 0, std::i16::MAX as isize);

    (step as i16, wait_cycles)
}
