pub mod display;
pub mod timing;
pub mod opengl;

use std::time::Duration;
use crate::constants::gpu::crtc::*;
use crate::State;
use crate::controllers::Event;
use crate::controllers::gpu::crtc::display::*;
use crate::controllers::gpu::crtc::timing::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => unsafe { run_time(state, time) },
    }
}

unsafe fn run_time(state: &State, duration: Duration) {
    let resources = &mut *state.resources;
    let vblank_time = &mut resources.gpu.crtc.vblank_time;

    let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        tick(state);
    }

    *vblank_time += duration;
    while *vblank_time >= REFRESH_RATE_NTSC_PERIOD {
        *vblank_time -= REFRESH_RATE_NTSC_PERIOD;
        
        unsafe {
            handle_vblank(state);
        }
    }
}

fn tick(state: &State) {
    unsafe {
        handle_timers(state);
    }
}
