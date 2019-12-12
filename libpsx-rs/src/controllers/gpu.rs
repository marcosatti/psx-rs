pub mod crtc;
pub mod command;
pub mod command_gp0;
pub mod command_gp0_impl;
pub mod command_gp1;
pub mod command_gp1_impl;
pub mod data;
pub mod opengl;

use std::time::Duration;
use crate::resources::Resources;
use crate::video::VideoBackend;
use crate::controllers::ControllerState;
use crate::constants::gpu::*;
use crate::controllers::Event;
use crate::controllers::gpu::command::*;
use crate::controllers::gpu::crtc::run_time as crtc_run_time;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, state.video_backend, time),
    }
}

fn run_time(resources: &mut Resources, video_backend: &VideoBackend, duration: Duration) {
    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(resources, video_backend);
    }

    crtc_run_time(resources, video_backend, duration);
}

fn tick(resources: &mut Resources, video_backend: &VideoBackend) {
    handle_command(resources, video_backend);
}
