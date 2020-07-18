use crate::system::{
    timers::{
        constants::*,
        controllers::{
            interrupt::*,
            timer::*,
        },
        types::*,
    },
    types::{ControllerResult, State},
};

pub(crate) fn handle_counter(state: &State, controller_state: &mut ControllerState, timer_id: usize) -> ControllerResult {
    let count = get_count(state, timer_id);
    let target = get_target(state, timer_id);
    let timer_state = get_state(controller_state, timer_id);

    let target_value = target.read_u32();
    let mut count_value = count.read_u32();
    let reset_on_target = timer_state.reset_on_target;
    let tick_period = calc_clock_source_period(timer_state.clock_source);

    while timer_state.clock > tick_period {
        count_value = (count_value + 1) & (std::u16::MAX as u32);

        // Check if timer has reached a reset/IRQ condition.
        if reset_on_target {
            if count_value == target_value {
                count_value = 0;
                timer_state.target_hit = true;
                handle_irq_trigger(state, timer_state, timer_id, IrqType::Target)?;
            }
        } else {
            if count_value == (std::u16::MAX as u32) {
                count_value = 0;
                timer_state.overflow_hit = true;
                handle_irq_trigger(state, timer_state, timer_id, IrqType::Overflow)?;
            }
        }

        count.write_u32(count_value);
        timer_state.clock -= tick_period;
    }

    Ok(())
}

const fn calc_clock_source_period(clock_source: ClockSource) -> f64 {
    match clock_source {
        ClockSource::Dotclock => DOTCLOCK_320_PERIOD_NTSC,
        ClockSource::Hblank => {
            // Timer ticks when HBLANK line is asserted... which happens after every scanline is rendered.
            // So we are actually ticking over when a scanline period has passed, in the context of an emulator.
            SCANLINE_PERIOD_NTSC
        },
        ClockSource::System => SYSTEM_CLOCK_PERIOD,
        ClockSource::System8 => SYSTEM_CLOCK_8_PERIOD,
    }
}
