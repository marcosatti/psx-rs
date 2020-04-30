use crate::system::{
    timers::{
        constants::*,
        controllers::{
            irq::*,
            timer::*,
        },
        types::*,
    },
    types::State,
};
use std::time::Duration;

pub fn handle_count(state: &State, timers_state: &mut ControllerState, timer_id: usize, duration: Duration) {
    let count = get_count(state, timer_id);
    let timer_state = get_state(timers_state, timer_id);

    timer_state.current_elapsed += duration;
    let delta_elapsed = timer_state.current_elapsed - timer_state.acknowledged_elapsed;
    let ticks = calc_ticks(timer_state.clock_source, delta_elapsed);
    timer_state.acknowledged_elapsed = timer_state.current_elapsed - ticks.1;

    for _ in 0..ticks.0 {
        let value = count.read_u32() + 1;
        count.write_u32(value);
        let irq_type = handle_count_reset(state, timer_id);
        handle_irq_trigger(state, timers_state, timer_id, irq_type);
    }
}

pub fn handle_count_clear(state: &State, timer_id: usize) {
    let count = get_count(state, timer_id);
    count.write_u32(0);
}

fn handle_count_reset(state: &State, timer_id: usize) -> IrqType {
    let mode = get_mode(state, timer_id);
    let count = get_count(state, timer_id);
    let count_value = count.read_u32() & 0xFFFF;

    let mut irq_type = IrqType::None;

    match mode.register.read_bitfield(MODE_RESET) {
        0 => {
            // When counter equals 0xFFFF.
            if count_value == (std::u16::MAX as u32) {
                handle_count_clear(state, timer_id);
                mode.register.write_bitfield(MODE_OVERFLOW_HIT, 1);
                irq_type = IrqType::Overflow;
            }
        },
        1 => {
            // When counter equals target.
            let target = get_target(state, timer_id);
            let target_value = target.read_u32() & 0xFFFF;
            if count_value == target_value {
                handle_count_clear(state, timer_id);
                mode.register.write_bitfield(MODE_TARGET_HIT, 0);
                irq_type = IrqType::Target;
            }
        },
        _ => unreachable!(),
    };

    irq_type
}

/// Given the clock source and difference in elapsed durations,
/// returns the number of whole ticks that have passed, with the remaining duration.
/// This is an approximate calculation, good enough for emulation purposes.
fn calc_ticks(clock_source: ClockSource, delta_elapsed: Duration) -> (usize, Duration) {
    let mut ticks = 0;
    let mut remaining_elapsed = delta_elapsed;

    let interval = match clock_source {
        ClockSource::Dotclock => DOTCLOCK_320_INTERVAL_NTSC,
        ClockSource::Hblank => {
            // Timer ticks when HBLANK line is asserted... which happens after every scanline is rendered.
            // So we are actually ticking over when a scanline interval has passed, in the context of an emulator.
            SCANLINE_INTERVAL_NTSC
        },
        ClockSource::System => SYSTEM_CLOCK_INTERVAL,
        ClockSource::System8 => SYSTEM_CLOCK_8_INTERVAL,
    };

    while remaining_elapsed > interval {
        remaining_elapsed -= interval;
        ticks += 1;
    }

    (ticks, remaining_elapsed)
}
