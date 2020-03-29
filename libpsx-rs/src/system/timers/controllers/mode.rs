use std::sync::atomic::Ordering;
use crate::system::types::State;
use crate::system::timers::types::*;
use crate::system::timers::controllers::timer::*;
use crate::system::timers::controllers::count::*;
use crate::system::timers::controllers::debug;
use crate::system::timers::constants::*;

pub fn handle_mode_write(state: &mut State, timer_id: usize) {
    let mode = get_mode(state, timer_id);

    if !mode.write_latch.load(Ordering::Acquire) {
        return;
    }

    let sync_mode = mode.register.read_bitfield(MODE_SYNC_EN);
    if sync_mode > 0 {
        unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
    }

    mode.register.write_bitfield(MODE_IRQ_PULSE, 1);

    handle_count_clear(state, timer_id);

    // Internal state initialization.
    handle_duration_clear(state, timer_id);
    handle_clock_source(state, timer_id);
    handle_oneshot_clear(state, timer_id);

    debug::trace_mode_write(state, timer_id);

    mode.write_latch.store(false, Ordering::Release);
}

pub fn handle_mode_read(state: &mut State, timer_id: usize) {
    let mode = get_mode(state, timer_id);

    if !mode.read_latch.load(Ordering::Acquire) {
        return;
    }

    mode.register.write_bitfield(MODE_OVERFLOW_HIT, 0);
    mode.register.write_bitfield(MODE_TARGET_HIT, 0);

    mode.write_latch.store(false, Ordering::Release);
}

pub fn handle_clock_source(state: &mut State, timer_id: usize) {
    let mode = get_mode(state, timer_id);
    let state = get_state(state, timer_id);

    let clock_source_value = mode.register.read_bitfield(MODE_CLK_SRC);
    
    let clock_source = match clock_source_value {
        0 => ClockSource::System,
        1 => {
            match timer_id {
                0 => ClockSource::Dotclock,
                1 => ClockSource::Hblank,
                2 => ClockSource::System,
                _ => unreachable!(),
            }
        },
        2 => {
            match timer_id {
                0 => ClockSource::System,
                1 => ClockSource::System,
                2 => ClockSource::System8,
                _ => unreachable!(),
            }
        },
        3 => {
            match timer_id {
                0 => ClockSource::Dotclock,
                1 => ClockSource::Hblank,
                2 => ClockSource::System8,
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    };

    state.clock_source = clock_source;
}


pub fn handle_oneshot_clear(state: &mut State, timer_id: usize) {
    let state = get_state(state, timer_id);
    state.irq_raised = false;
}
