pub mod count;
pub mod interrupt;
pub mod timer;
pub mod register;

use crate::system::{
    timers::controllers::{
        count::*,
        register::*,
    },
    types::{
        ControllerContext,
        Event,
        State,
    },
};
use std::time::Duration;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let controller_state = &mut state.timers.controller_state.lock();

    for timer_id in 0..3 {
        handle_mode(state, controller_state, timer_id);

        handle_counter(state, controller_state, timer_id, duration);
    }
}
