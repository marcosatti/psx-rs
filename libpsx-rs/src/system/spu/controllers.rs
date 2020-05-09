pub mod backend_dispatch;
pub mod dac;
pub mod transfer;
pub mod register;

use crate::{
    audio::AudioBackend,
    system::{
        spu::{
            constants::*,
            controllers::{
                dac::*,
                transfer::*,
                register::*,
            },
        },
        types::{
            ControllerContext,
            Event,
            State,
        },
    },
};
use std::time::Duration;
use std::cmp::max;

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, context.audio_backend, time),
    }
}

fn run_time(state: &State, audio_backend: &AudioBackend, duration: Duration) {
    let controller_state = &mut state.spu.controller_state.lock();

    if state.spu.voice_channel_fm.read_u32() > 0 {
        unimplemented!("Pitch modulation not implemented");
    }

    if state.spu.voice_channel_noise.read_u32() > 0 {
        unimplemented!("Noise generation not implemented");
    }

    let ticks = max(1, (CLOCK_SPEED * duration.as_secs_f64()) as i64);
    let dac_ratio = max(1, (CLOCK_SPEED / SAMPLE_RATE) as i64);

    for tick in 0..ticks {
        handle_control(state, controller_state);

        if !controller_state.enabled {
            continue;
        }

        handle_data_transfer_address(state, controller_state);
        handle_transfer(state, controller_state);
        handle_key_on(state, controller_state);
        handle_key_off(state, controller_state);

        if tick % dac_ratio == 0 {
            for voice_id in 0..24 {
                handle_dac(state, controller_state, audio_backend, voice_id);
            }
        }
    }
}
