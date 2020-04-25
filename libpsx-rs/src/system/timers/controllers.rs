pub mod count;
pub mod debug;
pub mod irq;
pub mod mode;
pub mod timer;
pub mod memory;

use crate::system::{
    timers::controllers::{
        count::*,
        mode::*,
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
    for i in 0..3 {
        let controller_state = state.timers.controller_state.try_lock().unwrap();

        handle_mode_write(state, controller_state, i);
        handle_mode_read(state, controller_state, i);
        handle_count(state, controller_state, i, duration);
    }
}
