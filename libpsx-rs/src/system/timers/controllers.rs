pub mod count;
pub mod debug;
pub mod interrupt;
pub mod timer;
pub mod resource;
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
        process_mode(state, controller_state, timer_id);

        handle_mode_write(state, timers_state, i);
        handle_mode_read(state, i);
        handle_count(state, timers_state, i, duration);
    }
}
