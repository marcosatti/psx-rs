pub mod adpcm;
pub mod adsr;
pub mod backend_dispatch;
pub mod dac;
pub mod interpolation;
pub mod interrupt;
pub mod sound;
pub mod transfer;
pub mod voice;
pub mod volume;

use crate::{
    audio::AudioBackend,
    system::{
        spu::{
            constants::*,
            controllers::{
                dac::*,
                interrupt::*,
                sound::*,
                transfer::*,
            },
            types::ControllerState,
        },
        types::{
            ControllerContext,
            Event,
            State,
        },
    },
};
use std::time::Duration;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, context.audio_backend, time),
    }
}

fn run_time(state: &State, audio_backend: &AudioBackend, duration: Duration) {
    {
        let control = &state.spu.control;

        if control.read_bitfield(CONTROL_ENABLE) == 0 {
            return;
        }
    }

    let spu_state = &mut state.spu.controller_state.lock();

    {
        let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

        for _ in 0..ticks {
            tick(state, spu_state);
        }
    }

    {
        handle_current_duration_tick(spu_state, duration);
        while handle_current_duration_update(spu_state) {
            generate_sound(state, spu_state, audio_backend);
        }
    }
}

fn tick(state: &State, spu_state: &mut ControllerState) {
    handle_current_volume(state);
    handle_transfer(state, spu_state);
    handle_interrupt_check(state);
}

fn handle_current_duration_tick(state: &mut ControllerState, duration: Duration) {
    let current_duration = &mut state.dac.current_duration;
    *current_duration += duration;
}

fn handle_current_duration_update(state: &mut ControllerState) -> bool {
    let current_duration = &mut state.dac.current_duration;

    if *current_duration >= SAMPLE_RATE_PERIOD {
        *current_duration -= SAMPLE_RATE_PERIOD;
        true
    } else {
        false
    }
}
