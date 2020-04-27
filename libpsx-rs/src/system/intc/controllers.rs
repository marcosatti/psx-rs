pub mod debug;
pub mod memory;

use crate::system::{
    intc::constants::CLOCK_SPEED,
    r3000::cp0::types::IrqLine,
    types::{
        ControllerContext,
        Event,
        State,
    },
};
use std::time::Duration;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &mut State, duration: Duration) {
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    for _ in 0..ticks {
        tick(state);
    }
}

fn tick(state: &State) {
    handle_interrupt_check(state);
}

fn handle_interrupt_check(state: &State) {
    let stat = &mut state.intc.stat;
    let mask = &mut state.intc.mask;

    let stat_value = stat.value();
    let mask_value = mask.read_u32();
    let masked_value = stat_value & mask_value;

    if masked_value == 0 {
        state.r3000.cp0.interrupt.deassert_line(IrqLine::Intc);
    } else {
        state.r3000.cp0.interrupt.assert_line(IrqLine::Intc);
    }
}
