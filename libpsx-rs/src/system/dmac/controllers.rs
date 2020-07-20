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

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult<()> {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: f64) -> ControllerResult<()> {
    let controller_state = &mut state.dmac.controller_state.lock();
    controller_state.clock += duration;

    let mut channel_id = 6;
    while controller_state.clock > 0.0 {
        handle_dpcr(state, controller_state)?;
        handle_dicr(state, controller_state)?;
        handle_chcr(state, controller_state, channel_id)?;

        let ticks_available = (controller_state.clock / CLOCK_SPEED_PERIOD) as usize;
        let (ticks_used, cooloff_required) = handle_transfer(state, controller_state, channel_id, ticks_available)?;
        controller_state.clock -= (ticks_used as f64) * CLOCK_SPEED_PERIOD;

        handle_irq_raise(state, controller_state)?;

        if cooloff_required {
            // Delay the DMAC a litte bit and allow the CPU to use the memory bus.
            controller_state.clock = -100.0 * CLOCK_SPEED_PERIOD;
        } else {
            if ticks_used <= 1 {
                // Channel had no meaningful progress made, so process the next one.
                channel_id = (channel_id + CHANNEL_COUNT - 1) % CHANNEL_COUNT;
            }
        }
    }

    Ok(())
}
