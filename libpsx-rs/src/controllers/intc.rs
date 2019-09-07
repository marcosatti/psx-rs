pub mod debug;

use std::time::Duration;
use crate::State;
use crate::resources::Resources;
use crate::constants::intc::CLOCK_SPEED;
use crate::controllers::Event;
use crate::resources::r3000::cp0::CAUSE_IP_INTC;

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

    let stat_value = stat.register.read_u32();
    let mask_value = mask.read_u32();

    let value = if (stat_value & mask_value) != 0 {
        1
    } else {
        0
    };

    let cause = &mut resources.r3000.cp0.cause;
    if cause.read_bitfield(CAUSE_IP_INTC) != value {
        let _cp0_lock = resources.r3000.cp0.mutex.lock();
        cause.write_bitfield(CAUSE_IP_INTC, value);
    }
}
