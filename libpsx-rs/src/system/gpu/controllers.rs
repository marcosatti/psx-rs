pub(crate) mod backend_dispatch;
pub(crate) mod command;
pub(crate) mod command_gp0;
pub(crate) mod command_gp0_impl;
pub(crate) mod command_gp1;
pub(crate) mod command_gp1_impl;
pub(crate) mod data;
pub(crate) mod debug;
pub(crate) mod read;
pub(crate) mod register;

use crate::{
    system::{
        gpu::{
            constants::*,
            controllers::{
                command::*,
                read::*,
                register::*,
            },
            crtc::controllers::run_time as crtc_run_time,
            types::ControllerState,
        },
        types::{
            ControllerContext,
            ControllerResult,
            Event,
            State,
        },
    },
    video::VideoBackend,
};

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult<()> {
    match event {
        Event::Time(time) => run_time(context.state, context.video_backend, time),
    }
}

fn run_time(state: &State, video_backend: &VideoBackend, duration: f64) -> ControllerResult<()> {
    let controller_state = &mut state.gpu.controller_state.lock();
    controller_state.clock += duration;

    while controller_state.clock > 0.0 {
        tick(state, controller_state, video_backend)?;
        controller_state.clock -= CLOCK_SPEED_NTSC_PERIOD;
    }

    crtc_run_time(state, video_backend, duration);

    Ok(())
}

fn tick(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend) -> ControllerResult<()> {
    handle_gp1(state, controller_state)?;

    handle_command(state, controller_state, video_backend)?;

    handle_read(state, controller_state)?;

    Ok(())
}
