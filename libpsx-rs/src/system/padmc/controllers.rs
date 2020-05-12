pub mod register;

use crate::system::{
    padmc::constants::*,
    padmc::controllers::register::*,
    padmc::types::*,
    types::{
        ControllerContext,
        Event,
        State,
    },
};
use std::{
    time::Duration,
};
use std::cmp::max;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, duration),
    }
}

fn run_time(state: &State, duration: Duration) {
    let controller_state = &mut state.padmc.controller_state.lock();

    let ticks = max(1, (CLOCK_SPEED * duration.as_secs_f64()) as isize);

    for _ in 0..ticks {
        tick(state, controller_state);
    }
}

pub fn tick(state: &State, controller_state: &mut ControllerState) {
    handle_stat(state);
    handle_ctrl(state, controller_state);

    if controller_state.tx_enabled {
        if let Ok(_) = state.padmc.tx_fifo.read_one() {
            state.padmc.rx_fifo.write_one(0xFF).unwrap();
        }
    }
}
