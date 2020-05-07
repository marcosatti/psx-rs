use crate::{
    system::{
        timers::types::*,
        types::State,
        timers::controllers::resource::*,
        timers::constants::*,
    },
};

pub fn process_mode(state: &State, controller_state: &mut ControllerState, timer_id: usize) {
    let value = match get_mode(state, timer_id).read_u32() {
        Ok(v) => v,
        Err(_) => return,
    };

    // Clear count register.
    get_count(state, timer_id).write_u32(0);

    // Reset and apply parameters.
    let mut timer_state = TimerState::new();

    let sync_mode = MODE_SYNC_EN.extract_from(value);
    if sync_mode > 0 {
        unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
    }

    let clock_source = match MODE_CLK_SRC.extract_from(value) {
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
}
