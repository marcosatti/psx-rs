pub(crate) mod count;
pub(crate) mod interrupt;
pub(crate) mod register;
pub(crate) mod timer;

use crate::system::{
    timers::controllers::{
        count::*,
        register::*,
        timer::*,
    },
    types::{
        ControllerContext,
        Event,
        State, ControllerResult,
    },
};

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: f64) -> ControllerResult {
    let controller_state = &mut state.timers.controller_state.lock();

    for timer_id in 0..3 {
        get_state(controller_state, timer_id).clock += duration;

        handle_mode(state, controller_state, timer_id);
        handle_counter(state, controller_state, timer_id);
    }

    Ok(())
}
