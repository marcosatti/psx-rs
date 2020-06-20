pub(crate) mod backend_dispatch;
pub(crate) mod command;
pub(crate) mod command_impl;
pub(crate) mod interrupt;
pub(crate) mod read;
pub(crate) mod register;
pub(crate) mod state;

use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::{
                command::*,
                read::*,
                register::{
                    handle_command as handle_command_register,
                    handle_interrupt_flag,
                    handle_request,
                },
            },
            types::ControllerState,
        },
        types::{
            ControllerContext,
            Event,
            State,
        },
    },
};

pub(crate) fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, context.cdrom_backend, duration),
    }
}

fn run_time(state: &State, cdrom_backend: &CdromBackend, duration: f64) {
    let controller_state = &mut state.cdrom.controller_state.lock();
    controller_state.clock += duration;

    while controller_state.clock > 0.0 {
        tick(state, controller_state, cdrom_backend);
        controller_state.clock -= CLOCK_SPEED_PERIOD;
    }
}

fn tick(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) {
    handle_command_register(state, controller_state);
    handle_request(state, controller_state);
    handle_interrupt_flag(state, controller_state);

    if controller_state.interrupt_index == 0 {
        handle_read(state, controller_state, cdrom_backend);
    }

    if controller_state.interrupt_index == 0 {
        handle_command(state, controller_state, cdrom_backend);
    }
}
