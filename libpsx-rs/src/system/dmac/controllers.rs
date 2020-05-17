pub(crate) mod channel;
pub(crate) mod interrupt;
pub(crate) mod register;
pub(crate) mod transfer;

use crate::system::{
    dmac::{
        constants::*,
        controllers::{
            interrupt::*,
            register::*,
            transfer::*,
        },
    },
    types::{
        ControllerContext,
        Event,
        State,
    },
};
use std::{
    cmp::max,
    time::Duration,
};

pub(crate) fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    // TODO: Properly obey priorities of channels.

    let controller_state = &mut state.dmac.controller_state.lock();

    // Don't run if the CPU needs to use the bus.
    if controller_state.cooloff_runs > 0 {
        controller_state.cooloff_runs -= 1;
        return;
    }

    let mut ticks = max(1, (CLOCK_SPEED * duration.as_secs_f64()) as isize);
    let mut cooloff = false;
    while (ticks > 0) && (!cooloff) {
        handle_dicr(state, controller_state);

        for channel_id in 0..7 {
            handle_chcr(state, controller_state, channel_id);

            match handle_transfer(state, controller_state, channel_id, &mut ticks) {
                Ok(()) => {},
                Err(()) => {
                    cooloff = true;
                    break;
                },
            }
        }

        handle_irq_raise(state, controller_state);
    }

    if cooloff {
        controller_state.cooloff_runs = 4;
    }
}
