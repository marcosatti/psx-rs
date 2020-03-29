pub mod backend_dispatch;
pub mod command;
pub mod command_gp0;
pub mod command_gp0_impl;
pub mod command_gp1;
pub mod command_gp1_impl;
pub mod data;
pub mod debug;

use crate::system::gpu::constants::*;
use crate::system::gpu::controllers::command::*;
use crate::system::gpu::crtc::controllers::run_time as crtc_run_time;
use crate::system::types::ControllerContext;
use crate::system::types::Event;
use crate::system::types::State;
use crate::video::VideoBackend;
use std::time::Duration;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, context.video_backend, time),
    }
}

fn run_time(state: &mut State, video_backend: &VideoBackend, duration: Duration) {
    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(state, video_backend);
    }

    crtc_run_time(state, video_backend, duration);
}

fn tick(state: &mut State, video_backend: &VideoBackend) {
    handle_command(state, video_backend);
}
