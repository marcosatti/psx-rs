pub mod count;
pub mod debug;
pub mod irq;
pub mod mode;
pub mod timer;

use crate::system::timers::controllers::count::*;
use crate::system::timers::controllers::mode::*;
use crate::system::types::ControllerContext;
use crate::system::types::Event;
use crate::system::types::State;
use std::time::Duration;

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
