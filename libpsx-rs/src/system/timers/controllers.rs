pub mod timer;
pub mod mode;
pub mod count;
pub mod irq;
pub mod debug;

use std::time::Duration;
use crate::system::types::ControllerContext;
use crate::system::types::State;
use crate::system::types::Event;
use crate::system::timers::controllers::mode::*;
use crate::system::timers::controllers::count::*;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &mut State, duration: Duration) {
    for i in 0..3 {
        handle_mode_write(state, i);
        handle_mode_read(state, i);
        handle_count(state, i, duration);
    }
}
