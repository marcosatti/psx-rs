pub mod backend_dispatch;
pub mod command;
pub mod command_gp0;
pub mod command_gp0_impl;
pub mod command_gp1;
pub mod command_gp1_impl;
pub mod data;
pub mod debug;
pub mod memory;

use crate::{
    system::{
        gpu::{
            constants::*,
            controllers::command::*,
            crtc::controllers::run_time as crtc_run_time,
            types::ControllerState,
        },
        types::{
            ControllerContext,
            Event,
            State,
        },
    },
    video::VideoBackend,
};
use std::time::Duration;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, context.video_backend, time),
    }
}

fn run_time(state: &State, video_backend: &VideoBackend, duration: Duration) {
    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;

    let gpu_state = &mut state.gpu.controller_state.lock();

    for _ in 0..ticks {
        tick(state, gpu_state, video_backend);
    }

    crtc_run_time(state, video_backend, duration);
}

fn tick(state: &State, gpu_state: &mut ControllerState, video_backend: &VideoBackend) {
    handle_command(state, gpu_state, video_backend);
}
