pub mod timer;

use std::sync::atomic::Ordering;
use std::time::Duration;
use log::debug;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::resources::timers::*;
use crate::constants::timers::CLOCK_SPEED;
use crate::controllers::Event;
use crate::controllers::timers::timer::*;

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
    for i in 0..3 {
        handle_mode(resources, i);
        handle_count(resources, i);
    }
}

fn handle_mode(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);

    if !mode.write_latch.load(Ordering::Acquire) {
        return;
    }

    let value = mode.register.read_u32();
    
    if MODE_SYNC_EN.extract_from(value) > 0 {
        let sync_mode = MODE_SYNC_MODE.extract_from(value);
        unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
    }

    handle_count_clear(resources, timer_id);

    debug!("Timer {} mode write acknowledged, cleared count", timer_id);
    mode.write_latch.store(false, Ordering::Release);
}

fn handle_count(resources: &mut Resources, timer_id: usize) {
    let count = get_count(resources, timer_id);
    
    let value = count.read_u32() + 1;
    count.write_u32(value);

    handle_count_reset(resources, timer_id);
}

fn handle_count_clear(resources: &mut Resources, timer_id: usize) {
    let count = get_count(resources, timer_id);
    count.write_u32(0);
}

fn handle_count_reset(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);
    let count = get_count(resources, timer_id);
    let count_value = count.read_u32();
    
    match mode.register.read_bitfield(MODE_RESET) {
        0 => {
            // When counter equals 0xFFFF.
            if count_value == (std::u16::MAX as u32) {
                handle_count_clear(resources, timer_id);
            }
        },
        1 => {
            // When counter equals target.
            let target = get_target(resources, timer_id);
            let target_value = target.read_u32() & 0xFFFF;
            if count_value == target_value {
                handle_count_clear(resources, timer_id);
                debug!("Cleared count for timer {} by target", timer_id);
            }
        },
        _ => unreachable!(),
    }
}
