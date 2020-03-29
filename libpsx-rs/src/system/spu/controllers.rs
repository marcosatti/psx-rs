pub mod voice;
pub mod transfer;
pub mod adpcm;
pub mod sound;
pub mod backend_dispatch;
pub mod dac;
pub mod volume;
pub mod adsr;
pub mod interpolation;
pub mod interrupt;

use std::time::Duration;
use crate::audio::AudioBackend;
use crate::system::types::ControllerContext;
use crate::system::types::State;
use crate::constants::spu::*;
use crate::constants::spu::dac::*;
use crate::system::types::Event;
use crate::controllers::spu::transfer::*;
use crate::controllers::spu::interrupt::*;
use crate::controllers::spu::dac::*;
use crate::controllers::spu::sound::*;
use crate::system::spu::*;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, state.audio_backend, time),
    }
}

fn run_time(state: &mut State, audio_backend: &AudioBackend, duration: Duration) {
    {
        let control = &resources.spu.control;

        if control.read_bitfield(CONTROL_ENABLE) == 0 {
            return;
        }
    }

    {
        let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

        for _ in 0..ticks {
            tick(resources);
        }
    }

    {
        handle_current_duration_tick(resources, duration);
        while handle_current_duration_update(resources) {
            generate_sound(resources, audio_backend);
        }
    }
}

fn tick(state: &mut State) {
    handle_current_volume(resources);
    handle_transfer(resources);
    handle_interrupt_check(resources);
}

fn handle_current_duration_tick(state: &mut State, duration: Duration) {
    let current_duration = &mut resources.spu.dac.current_duration;
    *current_duration += duration;
}

fn handle_current_duration_update(state: &mut State) -> bool {
    let current_duration = &mut resources.spu.dac.current_duration;

    if *current_duration >= SAMPLE_RATE_PERIOD {
        *current_duration -= SAMPLE_RATE_PERIOD;
        true
    } else {
        false
    }
}
