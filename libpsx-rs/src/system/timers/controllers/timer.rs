use crate::{
    system::{
        timers::types::*,
        types::State,
    },
    types::memory::*,
};
use std::time::Duration;

pub fn get_count(state: &State, timer_id: usize) -> &B32Register {
    match timer_id {
        0 => &state.timers.timer0_count,
        1 => &state.timers.timer1_count,
        2 => &state.timers.timer2_count,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_mode(state: &State, timer_id: usize) -> &Mode {
    match timer_id {
        0 => &state.timers.timer0_mode,
        1 => &state.timers.timer1_mode,
        2 => &state.timers.timer2_mode,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_target(state: &State, timer_id: usize) -> &B32Register {
    match timer_id {
        0 => &state.timers.timer0_target,
        1 => &state.timers.timer1_target,
        2 => &state.timers.timer2_target,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_state(timers_state: &mut ControllerState, timer_id: usize) -> &mut TimerState {
    match timer_id {
        0 => &mut timers_state.timer0_state,
        1 => &mut timers_state.timer1_state,
        2 => &mut timers_state.timer2_state,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn handle_duration_clear(timers_state: &mut ControllerState, timer_id: usize) {
    let state = get_state(timers_state, timer_id);
    state.current_elapsed = Duration::from_secs(0);
    state.acknowledged_elapsed = Duration::from_secs(0);
}

pub fn handle_oneshot_clear(timers_state: &mut ControllerState, timer_id: usize) {
    let state = get_state(timers_state, timer_id);
    state.irq_raised = false;
}
