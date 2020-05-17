use crate::{
    system::{
        timers::{
            constants::*,
            controllers::timer::*,
            types::*,
        },
        types::State,
    },
    types::memory::LatchKind,
    utilities::bool_to_flag,
};

pub(crate) fn handle_mode(state: &State, controller_state: &mut ControllerState, timer_id: usize) {
    get_mode(state, timer_id).acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => {
                let timer_state = get_state(controller_state, timer_id);

                timer_state.target_hit = false;
                timer_state.overflow_hit = false;

                calculate_mode_value(timer_state)
            },
            LatchKind::Write => {
                // Clear count register.
                get_count(state, timer_id).write_u32(0);

                // Reset and apply parameters.
                let timer_state = get_state(controller_state, timer_id);

                let sync_mode = MODE_SYNC_EN.extract_from(value);
                if sync_mode > 0 {
                    unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
                }

                timer_state.reset_on_target = MODE_RESET.extract_from(value) > 0;

                timer_state.irq_on_target = MODE_IRQ_TARGET.extract_from(value) > 0;

                timer_state.irq_on_overflow = MODE_IRQ_OVERFLOW.extract_from(value) > 0;

                timer_state.oneshot_mode = MODE_IRQ_REPEAT.extract_from(value) == 0;

                timer_state.irq_toggle = MODE_IRQ_PULSE.extract_from(value) > 0;

                timer_state.clock_source_raw = MODE_CLK_SRC.extract_from(value);

                timer_state.clock_source = match timer_state.clock_source_raw {
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

                if MODE_IRQ_STATUS.extract_from(value) > 0 {
                    timer_state.irq_raised = false;
                }

                calculate_mode_value(timer_state)
            },
        }
    });
}

pub(crate) fn calculate_mode_value(timer_state: &TimerState) -> u32 {
    let mut value = 0;

    value = MODE_RESET.insert_into(value, bool_to_flag(timer_state.reset_on_target));
    value = MODE_IRQ_TARGET.insert_into(value, bool_to_flag(timer_state.irq_on_target));
    value = MODE_IRQ_OVERFLOW.insert_into(value, bool_to_flag(timer_state.irq_on_overflow));
    value = MODE_IRQ_REPEAT.insert_into(value, bool_to_flag(!timer_state.oneshot_mode));
    value = MODE_IRQ_PULSE.insert_into(value, bool_to_flag(timer_state.irq_toggle));
    value = MODE_CLK_SRC.insert_into(value, timer_state.clock_source_raw);
    value = MODE_IRQ_STATUS.insert_into(value, bool_to_flag(timer_state.irq_raised));
    value = MODE_TARGET_HIT.insert_into(value, bool_to_flag(timer_state.target_hit));
    value = MODE_OVERFLOW_HIT.insert_into(value, bool_to_flag(timer_state.overflow_hit));

    value
}
