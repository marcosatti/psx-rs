pub mod command;

use std::time::Duration;
use crate::State;
use crate::constants::cdrom::CLOCK_SPEED;
use crate::controllers::Event;
use crate::controllers::cdrom::command::*;
use crate::resources::cdrom::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        unsafe { handle_tick(state) };
    }
}

unsafe fn handle_tick(state: &State) {
    handle_parameter_fifo(state);

    handle_command(state);
}

unsafe fn handle_parameter_fifo(state: &State) {
    let resources = &mut *state.resources;
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.parameter;

    let empty_bit = if fifo.is_empty() {
        1
    } else {
        0
    };

    status.write_bitfield(STATUS_PRMEMPT, empty_bit);

    let ready_bit = if !fifo.is_full() {
        1
    } else {
        0
    };

    status.write_bitfield(STATUS_PRMWRDY, ready_bit);
}
