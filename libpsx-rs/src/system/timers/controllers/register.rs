use crate::{
    system::{
        timers::types::*,
        types::State,
        timers::controllers::timer::*,
        timers::constants::*,
    },
    types::memory::LatchKind,
};

pub fn process_mode(state: &State, controller_state: &mut ControllerState, timer_id: usize) {
    let mut write_fn = |value| {
        // Clear count register.
        get_count(state, timer_id).write_u32(0);

        // Reset and apply parameters.
        let mut timer_state = TimerState::new();

        let sync_mode = MODE_SYNC_EN.extract_from(value);
        if sync_mode > 0 {
            unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
        }

        timer_state.reset_on_target = MODE_RESET.extract_from(value) > 0;
        
        timer_state.irq_on_target = MODE_IRQ_TARGET.extract_from(value) > 0;
        
        timer_state.irq_on_overflow = MODE_IRQ_OVERFLOW.extract_from(value) > 0;
        
        timer_state.oneshot_mode = MODE_IRQ_REPEAT.extract_from(value) > 0;

        timer_state.irq_toggle = MODE_IRQ_PULSE.extract_from(value) > 0;

        timer_state.clock_source = match MODE_CLK_SRC.extract_from(value) {
            0 => ClockSource::System,
            1 => match timer_id {
                0 => ClockSource::Dotclock,
                1 => ClockSource::Hblank,
                2 => ClockSource::System,
                _ => unreachable!(),
            },
            2 => match timer_id {
                0 => ClockSource::System,
                1 => ClockSource::System,
                2 => ClockSource::System8,
                _ => unreachable!(),
            },
            3 => match timer_id {
                0 => ClockSource::Dotclock,
                1 => ClockSource::Hblank,
                2 => ClockSource::System8,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        *get_state(controller_state, timer_id) = timer_state;
    };

    let read_fn = |mut value| {
        value = MODE_OVERFLOW_HIT.insert_into(value, 0);
        value = MODE_TARGET_HIT.insert_into(value, 0);
        value
    };

    get_mode(state, timer_id).acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => read_fn(value),
            LatchKind::Write => { write_fn(value); value },
        }
    });
}
