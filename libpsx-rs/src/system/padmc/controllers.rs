pub(crate) mod register;

use crate::system::{
    padmc::{
        constants::*,
        controllers::register::*,
        types::*,
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
        Event::Time(duration) => run_time(context.state, duration),
    }
}

fn run_time(state: &State, duration: f64) -> ControllerResult {
    let controller_state = &mut state.padmc.controller_state.lock();
    controller_state.clock += duration;

    while controller_state.clock > 0.0 {
        tick(state, controller_state)?;
        controller_state.clock -= CLOCK_SPEED_PERIOD;
    }

    Ok(())
}

pub(crate) fn tick(state: &State, controller_state: &mut ControllerState) -> ControllerResult {
    handle_ctrl(state, controller_state)?;

    if controller_state.tx_enabled {
        if let Ok(_) = state.padmc.tx_fifo.read_one() {
            state.padmc.rx_fifo.write_one(0xFF).map_err(|_| "Error pushing to RX FIFO".to_owned())?;
        }
    }

    Ok(())
}
