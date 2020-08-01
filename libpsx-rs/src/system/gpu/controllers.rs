pub(crate) mod backend_dispatch;
pub(crate) mod command;
pub(crate) mod command_gp0;
pub(crate) mod command_gp0_impl;
pub(crate) mod command_gp1;
pub(crate) mod command_gp1_impl;
pub(crate) mod data;
pub(crate) mod debug;
pub(crate) mod read;

use crate::{
    system::{
        gpu::{
            constants::*,
            controllers::{
                command::*,
                read::*,
            },
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

    Ok(())
}

fn tick(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend) -> ControllerResult<()> {
    let mut handled = false;

    if !handled {
        handled = handle_command(state, controller_state, video_backend)?;
    }

    if !handled {
        handled = handle_read(state, controller_state)?;
    }

    if !handled {
    }

    Ok(())
}
