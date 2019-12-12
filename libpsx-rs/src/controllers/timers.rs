pub mod timer;
pub mod mode;
pub mod count;
pub mod irq;
pub mod debug;

use std::time::Duration;
use log::debug;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::controllers::Event;
use crate::controllers::timers::mode::*;
use crate::controllers::timers::count::*;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    for i in 0..3 {
        handle_mode_write(resources, i);
        handle_mode_read(resources, i);
        handle_count(resources, i, duration);
    }
}
