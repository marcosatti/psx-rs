use crate::system::{
    timers::{
        constants::*,
        controllers::{
            interrupt::*,
            timer::*,
        },
        types::*,
    },
    types::State,
};
use std::time::Duration;

pub fn handle_counter(state: &State, controller_state: &mut ControllerState, timer_id: usize, duration: Duration) {
    let count = get_count(state, timer_id);
    let target = get_target(state, timer_id);

    let ticks = {
        let timer_state = get_state(controller_state, timer_id);
        timer_state.current_elapsed += duration;
        let delta_elapsed = timer_state.current_elapsed - timer_state.acknowledged_elapsed;
        let ticks = calc_ticks(timer_state.clock_source, delta_elapsed);
        timer_state.acknowledged_elapsed = timer_state.current_elapsed - ticks.1;
        ticks.0
    };

    let reset_on_target = get_state(controller_state, timer_id).reset_on_target;
    let target_value = target.read_u32();
    let mut count_value = count.read_u32();

    for _ in 0..ticks {
        count_value = (count_value + 1) & (std::u16::MAX as u32);

        // Check if timer has reached a reset/IRQ condition.
        if reset_on_target {
            if count_value == target_value {
                count_value = 0;
                get_state(controller_state, timer_id).target_hit = true;
                handle_irq_trigger(state, controller_state, timer_id, IrqType::Target);
            }
        } else {
            if count_value == (std::u16::MAX as u32) {
                count_value = 0;
                get_state(controller_state, timer_id).overflow_hit = true;
                handle_irq_trigger(state, controller_state, timer_id, IrqType::Overflow);
            }
        }

        count.write_u32(count_value);
    }
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
