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

pub(crate) fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: f64) {
    // TODO: Properly obey priorities of channels.

    let controller_state = &mut state.dmac.controller_state.lock();
    controller_state.clock += duration;

    // Don't run if the CPU needs to use the bus.
    if controller_state.cooloff_runs > 0 {
        controller_state.clock = 0.0;
        controller_state.cooloff_runs -= 1;
        return;
    }

    let mut cooloff = false;
    while (controller_state.clock > 0.0) && (!cooloff) {
        handle_dicr(state, controller_state);

        for channel_id in 0..7 {
            handle_chcr(state, controller_state, channel_id);

            let ticks = (controller_state.clock / CLOCK_SPEED_PERIOD) as isize;
            let mut ticks_remaining = ticks;

            match handle_transfer(state, controller_state, channel_id, &mut ticks_remaining) {
                Ok(()) => {},
                Err(()) => cooloff = true,
            }

            controller_state.clock -= ((ticks - ticks_remaining) as f64) * CLOCK_SPEED_PERIOD;

            if cooloff {
                break;
            }
        }

        handle_irq_raise(state, controller_state);
    }

    if cooloff {
        controller_state.cooloff_runs = 4;
    }
}
