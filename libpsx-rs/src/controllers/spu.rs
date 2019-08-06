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
        Event::Time(time) => unsafe { run_time(state, time) },
    }
}

unsafe fn run_time(state: &State, duration: Duration) {
    let resources = &mut *state.resources;
    let control = &resources.spu.control;

    if control.read_bitfield(CONTROL_ENABLE) == 0 {
        return;
    }

    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(state);
    }

    let current_duration = &mut resources.spu.dac.current_duration;

    *current_duration += duration;
    while *current_duration >= SAMPLE_RATE_PERIOD {
        *current_duration -= SAMPLE_RATE_PERIOD;
        generate_sound(state);
    }
}

fn tick(state: &State) {
    unsafe {
        handle_current_volume(state);
        handle_transfer(state);
        handle_interrupt_check(state);
    }
}
