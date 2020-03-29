pub mod crtc;
pub mod command;
pub mod command_gp0;
pub mod command_gp0_impl;
pub mod command_gp1;
pub mod command_gp1_impl;
pub mod data;
pub mod backend_dispatch;
pub mod debug;

use std::time::Duration;
use crate::system::types::State;
use crate::video::VideoBackend;
use crate::system::types::ControllerContext;
use crate::constants::gpu::*;
use crate::system::types::Event;
use crate::controllers::gpu::command::*;
use crate::controllers::gpu::crtc::run_time as crtc_run_time;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, state.video_backend, time),
    }
}

fn run_time(state: &mut State, video_backend: &VideoBackend, duration: Duration) {
    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(resources, video_backend);
    }

    crtc_run_time(resources, video_backend, duration);
}

fn tick(state: &mut State, video_backend: &VideoBackend) {
    handle_command(resources, video_backend);
}
