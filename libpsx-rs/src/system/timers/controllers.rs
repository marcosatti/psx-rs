pub mod count;
pub mod debug;
pub mod irq;
pub mod memory;
pub mod mode;
pub mod timer;

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
        let timers_state = &mut state.timers.controller_state.lock();

        handle_mode_write(state, timers_state, i);
        handle_mode_read(state, i);
        handle_count(state, timers_state, i, duration);
    }
}
