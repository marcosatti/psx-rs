pub mod debug;

use std::time::Duration;
use crate::controllers::ControllerState;
use crate::types::bitfield::Bitfield;
use crate::resources::Resources;
use crate::constants::intc::CLOCK_SPEED;
use crate::controllers::Event;
use crate::resources::r3000::cp0::register::IrqLine;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
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
        
        if masked_value == 0 {
            cause.reset_irq(IrqLine::Intc);
        } else {
            for i in 0..32 {
                if is_edge_triggered(i, *old_masked_value, masked_value) {
                    cause.raise_irq(IrqLine::Intc);
                    break;
                }
            }
        }

        *old_masked_value = masked_value;
    }
}

/// Checks for a 0 -> 1 transition.
fn is_edge_triggered(bit: usize, old_value: u32, new_value: u32) -> bool {
    let bit = Bitfield::new(bit, 1);
    let old_value = bit.extract_from(old_value) > 0;
    let new_value = bit.extract_from(new_value) > 0;
    (!old_value) && (new_value)
}
