pub mod crtc;
pub mod command;
pub mod command_gp0;
pub mod command_gp1;
pub mod data;
pub mod opengl;

use std::time::Duration;
use crate::State;
use crate::constants::gpu::*;
use crate::controllers::Event;
use crate::controllers::gpu::command::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(state);
    }
}

fn tick(state: &State) {
    unsafe {
        handle_command(state);
    }
}
