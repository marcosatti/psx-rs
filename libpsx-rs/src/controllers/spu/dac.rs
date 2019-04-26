use std::time::Duration;
use crate::constants::spu::dac::*;
use crate::State;
use crate::controllers::Event;
use crate::controllers::spu::sound::*;
use crate::resources::spu::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    unsafe {
        tick(state, duration);
    }
}

unsafe fn tick(state: &State, duration: Duration) {
    let resources = &mut *state.resources;
    let control = &resources.spu.control;
    let current_duration = &mut resources.spu.dac.current_duration;

    *current_duration += duration;
    if *current_duration > SAMPLE_RATE_PERIOD {
        *current_duration -= SAMPLE_RATE_PERIOD;

        if control.read_bitfield(CONTROL_ENABLE) == 0 {
            return;
        }

        if control.read_bitfield(CONTROL_MUTE) == 0 {
            return;
        }

        generate_sound(state);

        handle_current_volume(state);
    }
}

unsafe fn handle_current_volume(state: &State) {
    let resources = &mut *state.resources;
    let main_volume_left = &mut resources.spu.main_volume_left;
    let main_volume_right = &mut resources.spu.main_volume_right;
    let current_volume_left = &mut resources.spu.current_volume_left;
    let current_volume_right = &mut resources.spu.current_volume_right;

    current_volume_left.write_u16(main_volume_left.read_u16());
    current_volume_right.write_u16(main_volume_right.read_u16());
}
