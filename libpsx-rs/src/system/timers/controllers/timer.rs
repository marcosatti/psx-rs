use crate::{
    system::{
        timers::types::*,
        types::State,
    },
    types::memory::*,
};

pub fn get_count(state: &State, timer_id: usize) -> &B32LevelRegister {
    match timer_id {
        0 => &state.timers.timer0_count,
        1 => &state.timers.timer1_count,
        2 => &state.timers.timer2_count,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_mode(state: &State, timer_id: usize) -> &B32EdgeRegister {
    match timer_id {
        0 => &state.timers.timer0_mode,
        1 => &state.timers.timer1_mode,
        2 => &state.timers.timer2_mode,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_target(state: &State, timer_id: usize) -> &B32LevelRegister {
    match timer_id {
        0 => &state.timers.timer0_target,
        1 => &state.timers.timer1_target,
        2 => &state.timers.timer2_target,
        _ => unreachable!("Invalid timer ID"),
    }
}

pub fn get_state(controller_state: &mut ControllerState, timer_id: usize) -> &mut TimerState {
    match timer_id {
        0 => &mut controller_state.timer0_state,
        1 => &mut controller_state.timer1_state,
        2 => &mut controller_state.timer2_state,
        _ => unreachable!("Invalid timer ID"),
    }
}
