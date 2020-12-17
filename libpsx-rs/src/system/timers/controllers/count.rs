use crate::system::{
    timers::{
        constants::*,
        controllers::{
            interrupt::*,
            timer::*,
        },
        types::*,
    },
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn handle_counter(state: &State, controller_state: &mut ControllerState, timer_id: usize) -> ControllerResult<()> {
    let count = get_count(state, timer_id);
    let target = get_target(state, timer_id);
    let timer_state = get_state(controller_state, timer_id);

    let target_value = target.read_u32();
    let mut count_value = count.read_u32();
    let tick_period = calc_clock_source_period(timer_state.clock_source);

    while timer_state.clock > 0.0 {
        count_value = (count_value + 1) & (std::u16::MAX as u32);

        let hblank_current = get_hblank(state, timer_id).load();
        let vblank_current = get_vblank(state, timer_id).load();

        match timer_state.sync_mode {
            SyncMode::Off => {},
            SyncMode::HblankReset => {
                // TODO: properly implement.
                // if (!timer_state.hblank_old) && hblank_current {
                //     count_value = 0;
                // }
                if hblank_current {
                    get_hblank(state, timer_id).store(false);
                    count_value = 0;
                }
            },
            SyncMode::HblankPauseOff => {
                if !hblank_current {
                    timer_state.clock -= tick_period;
                    continue;
                } else {
                    get_hblank(state, timer_id).store(false);
                    timer_state.sync_mode = SyncMode::Off;
                }
            },
            SyncMode::VblankReset => {
                // TODO: properly implement.
                // if (!timer_state.vblank_old) && vblank_current {
                //     count_value = 0;
                // }
                if vblank_current {
                    get_vblank(state, timer_id).store(false);
                    count_value = 0;
                }
            },
            SyncMode::VblankPauseOff => {
                if !vblank_current {
                    timer_state.clock -= tick_period;
                    continue;
                } else {
                    get_vblank(state, timer_id).store(false);
                    timer_state.sync_mode = SyncMode::Off;
                }
            },
            _ => return Err(format!("Sync mode {:?} not implemented", timer_state.sync_mode)),
        }

        timer_state.hblank_old = hblank_current;
        timer_state.vblank_old = vblank_current;

        count.write_u32(count_value);

        // Check if timer has reached a reset/IRQ condition.
        if timer_state.reset_on_target {
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

        timer_state.clock -= tick_period;
    }

    Ok(())
}

const fn calc_clock_source_period(clock_source: ClockSource) -> f32 {
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
