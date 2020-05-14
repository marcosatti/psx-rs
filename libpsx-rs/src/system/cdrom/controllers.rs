pub mod backend_dispatch;
pub mod register;
pub mod interrupt;
pub mod command_impl;
pub mod command;
pub mod state;
pub mod read;

use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::register::{handle_command as handle_command_register, handle_request, handle_interrupt_flag},
            types::ControllerState,
            controllers::command::*,
            controllers::read::*,
        },
        types::State,
        types::Event,
        types::ControllerContext,
    },
};
use std::cmp::max;
use std::time::Duration;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, context.cdrom_backend, duration),
    }
}

fn run_time(state: &State, cdrom_backend: &CdromBackend, duration: Duration) {
    let controller_state = &mut state.cdrom.controller_state.lock();

    let ticks = max(1, (CLOCK_SPEED * duration.as_secs_f64()) as isize);

    for _ in 0..ticks {
        tick(state, controller_state, cdrom_backend);
    }
}

fn tick(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) {
    handle_command_register(state, controller_state);
    handle_request(state, controller_state);
    handle_interrupt_flag(state, controller_state);

    //  check for incoming sectors (from CDROM decoder) ; only if no int (sort of)
    //   check for incoming commands (from Main CPU) ; only if no int
    //   do maintenance stuff on the drive mechanics ; n/a

    // Main CPU has sent a command, AND, there is no INT pending
    // (if an INT is pending, then the command won't be executed yet, but will be
    // executed in following mainloop cycles; once when INT got acknowledged)
    // (even if no INT is pending, the mainloop may generate INT1/INT2 before
    // executing the command, if so, as said above, the command won't execute yet)

    if controller_state.interrupt_index == 0 {
        handle_read(state, controller_state, cdrom_backend);
    }

    if controller_state.interrupt_index == 0 {
        handle_command(state, controller_state, cdrom_backend);
    }
}
