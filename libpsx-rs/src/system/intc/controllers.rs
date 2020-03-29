pub mod debug;

use std::time::Duration;
use crate::system::types::ControllerContext;
use crate::system::types::State;
use crate::constants::intc::CLOCK_SPEED;
use crate::system::types::Event;
use crate::system::r3000::cp0::register::IrqLine;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(state: &mut State, duration: Duration) {
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    for _ in 0..ticks {
        tick(resources);
    }
}

fn tick(state: &mut State) {
    handle_interrupt_check(resources);
}

fn handle_interrupt_check(state: &mut State) {
    let stat = &mut resources.intc.stat;
    let mask = &mut resources.intc.mask;

    let stat_value = stat.value();
    let mask_value = mask.read_u32();
    let masked_value = stat_value & mask_value;

    if masked_value == 0 {
        resources.r3000.cp0.cause.deassert_line(IrqLine::Intc);
    } else {
        resources.r3000.cp0.cause.assert_line(IrqLine::Intc);
    }
}
