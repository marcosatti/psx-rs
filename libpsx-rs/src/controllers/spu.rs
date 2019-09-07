pub mod voice;
pub mod transfer;
pub mod adpcm;
pub mod sound;
pub mod openal;
pub mod dac;
pub mod volume;
pub mod adsr;
pub mod interpolation;
pub mod interrupt;

use std::time::Duration;
use crate::State;
use crate::resources::Resources;
use crate::constants::spu::*;
use crate::constants::spu::dac::*;
use crate::controllers::Event;
use crate::controllers::spu::transfer::*;
use crate::controllers::spu::interrupt::*;
use crate::controllers::spu::dac::*;
use crate::controllers::spu::sound::*;
use crate::resources::spu::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    {
        let resources = unsafe { &mut *state.resources };
        let control = &resources.spu.control;

        if control.read_bitfield(CONTROL_ENABLE) == 0 {
            return;
        }
    }

    {
        let resources = unsafe { &mut *state.resources };
        let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

        for _ in 0..ticks {
            tick(resources);
        }
    }

    {
        let resources = unsafe { &mut *state.resources };
        let current_duration = unsafe { &mut *(&mut resources.spu.dac.current_duration as *mut Duration) };
        
        *current_duration += duration;
        while *current_duration >= SAMPLE_RATE_PERIOD {
            *current_duration -= SAMPLE_RATE_PERIOD;
            generate_sound(resources, state.audio_backend);
        }
    }
}

fn tick(resources: &mut Resources) {
    handle_current_volume(resources);
    handle_transfer(resources);
    handle_interrupt_check(resources);
}
