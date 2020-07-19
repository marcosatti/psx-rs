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
        ControllerResult,
        Event,
        State,
    },
};

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: f64) -> ControllerResult {
    // TODO: Properly obey priorities of channels.

    let controller_state = &mut state.dmac.controller_state.lock();
    controller_state.clock += duration;

    // Don't run if the CPU needs to use the bus.
    if controller_state.cooloff_runs > 0 {
        controller_state.clock = 0.0;
        controller_state.cooloff_runs -= 1;
        return Ok(());
    }

    let mut channel_id = 0;
    while controller_state.clock > 0.0 {
        handle_dicr(state, controller_state)?;
        handle_chcr(state, controller_state, channel_id)?;

        let ticks_available = (controller_state.clock / CLOCK_SPEED_PERIOD) as usize;
        let (ticks_used, cooloff) = handle_transfer(state, controller_state, channel_id, ticks_available)?;
        controller_state.clock -= (ticks_used as f64) * CLOCK_SPEED_PERIOD;

        handle_irq_raise(state, controller_state)?;
        channel_id = (channel_id + 1) % CHANNEL_COUNT;

        if cooloff {
            controller_state.cooloff_runs = 4;
            break;
        }
    }

    Ok(())
}
