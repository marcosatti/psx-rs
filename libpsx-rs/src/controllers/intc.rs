pub mod debug;

use log::debug;
use std::time::Duration;
use crate::State;
use crate::resources::Resources;
use crate::constants::intc::CLOCK_SPEED;
use crate::controllers::Event;
use crate::resources::r3000::cp0::register::IrqLine;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let resources = unsafe { &mut *state.resources };
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    for _ in 0..ticks {
        tick(resources);
    }
}

fn tick(resources: &mut Resources) {
    handle_interrupt_check(resources);
}

fn handle_interrupt_check(resources: &mut Resources) {
    let stat = &mut resources.intc.stat;
    let mask = &mut resources.intc.mask;
    let old_masked_value = &mut resources.intc.old_masked_value;

    let stat_value = stat.register.read_u32();
    let mask_value = mask.read_u32();
    let masked_value = stat_value & mask_value;

    if masked_value != *old_masked_value {
        let cause = &resources.r3000.cp0.cause;

        debug!("INTC edge triggered event, value = 0x{:08X}", masked_value);

        if masked_value != 0 { 
            cause.raise_irq(IrqLine::Intc);
        } else { 
            cause.reset_irq(IrqLine::Intc);
        }

        *old_masked_value = masked_value;
    }
}
