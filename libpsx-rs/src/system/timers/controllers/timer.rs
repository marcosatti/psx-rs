use crate::{
    system::{
        timers::types::*,
    },
};
use std::time::Duration;
use crate::system::timers::controllers::resource::*;

pub fn handle_duration_clear(timers_state: &mut ControllerState, timer_id: usize) {
    let state = get_state(timers_state, timer_id);
    state.current_elapsed = Duration::from_secs(0);
    state.acknowledged_elapsed = Duration::from_secs(0);
}

pub fn handle_oneshot_clear(timers_state: &mut ControllerState, timer_id: usize) {
    let state = get_state(timers_state, timer_id);
    state.irq_raised = false;
}
