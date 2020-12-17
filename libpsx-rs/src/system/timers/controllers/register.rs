use crate::{
    system::{
        timers::{
            constants::*,
            controllers::timer::*,
            types::*,
        },
        types::{
            ControllerResult,
            State,
        },
    },
    types::memory::LatchKind,
};

pub(crate) fn handle_mode(state: &State, controller_state: &mut ControllerState, timer_id: usize) -> ControllerResult<()> {
    get_mode(state, timer_id).acknowledge(|mut value, latch_kind| {
        match latch_kind {
            LatchKind::Read => {
                let timer_state = get_state(controller_state, timer_id);

                timer_state.target_hit = false;
                value = MODE_TARGET_HIT.insert_into(value, 0);

                timer_state.overflow_hit = false;
                value = MODE_OVERFLOW_HIT.insert_into(value, 0);

                Ok(value)
            },
            LatchKind::Write => {
                // Clear count register.
                get_count(state, timer_id).write_u32(0);

                // Reset and apply parameters.
                let timer_state = get_state(controller_state, timer_id);

                timer_state.sync_mode = if MODE_SYNC_ENABLE.extract_from(value) > 0 {
                    calculate_sync_mode(MODE_SYNC_MODE.extract_from(value), timer_id)?
                } else {
                    SyncMode::Off
                };

                timer_state.reset_on_target = MODE_RESET.extract_from(value) > 0;

                timer_state.irq_on_target = MODE_IRQ_TARGET.extract_from(value) > 0;

                timer_state.irq_on_overflow = MODE_IRQ_OVERFLOW.extract_from(value) > 0;

                timer_state.oneshot_mode = MODE_IRQ_REPEAT.extract_from(value) == 0;

                timer_state.irq_toggle = MODE_IRQ_PULSE.extract_from(value) > 0;

                timer_state.clock_source = calculate_clock_source(MODE_CLK_SRC.extract_from(value), timer_id)?;

                if MODE_IRQ_STATUS.extract_from(value) > 0 {
                    timer_state.irq_raised = false;
                }

                Ok(value)
            },
        }
    })
}

fn calculate_sync_mode(sync_mode_raw: u32, timer_id: usize) -> ControllerResult<SyncMode> {
    let sync_mode = match timer_id {
        0 => {
            match sync_mode_raw {
                0 => SyncMode::HblankPause,
                1 => {
                    log::warn!("HblankReset sync mode not properly implemented (needs CRTC & Timers work)");
                    SyncMode::HblankReset
                },
                2 => SyncMode::HblankResetPause,
                3 => {
                    log::warn!("HblankPauseOff sync mode not properly implemented (needs CRTC & Timers work)");
                    SyncMode::HblankPauseOff
                },
                _ => return Err(format!("Invalid sync mode: {}", sync_mode_raw)),
            }
        },
        1 => {
            match sync_mode_raw {
                0 => SyncMode::VblankPause,
                1 => {
                    log::warn!("VblankReset sync mode not properly implemented (needs CRTC & Timers work)");
                    SyncMode::VblankReset
                },
                2 => SyncMode::VblankResetPause,
                3 => {
                    log::warn!("VblankPauseOff sync mode not properly implemented (needs CRTC & Timers work)");
                    SyncMode::VblankPauseOff
                },
                _ => return Err(format!("Invalid sync mode: {}", sync_mode_raw)),
            }
        },
        2 => {
            match sync_mode_raw {
                0 | 3 => SyncMode::Stop,
                1 | 2 => SyncMode::Off,
                _ => return Err(format!("Invalid sync mode: {}", sync_mode_raw)),
            }
        },
        _ => return Err(format!("Invalid timer ID: {}", timer_id)),
    };

    Ok(sync_mode)
}

fn calculate_clock_source(clock_source_raw: u32, timer_id: usize) -> ControllerResult<ClockSource> {
    let clock_source = match clock_source_raw {
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

    Ok(clock_source)
}
