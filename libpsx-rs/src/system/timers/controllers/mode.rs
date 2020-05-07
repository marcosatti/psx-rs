use crate::system::{
    timers::{
        constants::*,
        controllers::{
            count::*,
            debug,
            timer::*,
        },
        types::*,
    },
    types::State,
};
use std::sync::atomic::Ordering;

pub fn handle_mode_read(state: &State, timer_id: usize) {
    let mode = get_mode(state, timer_id);

    if !mode.read_latch.load(Ordering::Acquire) {
        return;
    }

    mode.register.write_bitfield(MODE_OVERFLOW_HIT, 0);
    mode.register.write_bitfield(MODE_TARGET_HIT, 0);

    mode.write_latch.store(false, Ordering::Release);
}
