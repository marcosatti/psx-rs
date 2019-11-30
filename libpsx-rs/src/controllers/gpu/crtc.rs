pub mod display;
pub mod timing;
pub mod opengl;

use std::time::Duration;
use crate::video::VideoBackend;
use crate::resources::Resources;
use crate::constants::gpu::crtc::*;
use crate::controllers::ControllerState;
use crate::controllers::Event;
use crate::controllers::gpu::crtc::display::*;
use crate::controllers::gpu::crtc::timing::*;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, state.video_backend, time),
    }
}

fn run_time(resources: &mut Resources, video_backend: &VideoBackend, duration: Duration) {
    {
        let ticks = (CLOCK_SPEED_NTSC * duration.as_secs_f64()) as i64;
        for _ in 0..ticks {
            tick(resources);
        }
    }

    {
        let vblank_time = unsafe { &mut *(&mut resources.gpu.crtc.vblank_time as *mut Duration) };

        *vblank_time += duration;
        while *vblank_time >= REFRESH_RATE_NTSC_PERIOD {
            *vblank_time -= REFRESH_RATE_NTSC_PERIOD;
            handle_vblank(resources, video_backend);
        }
    }
}

fn tick(resources: &mut Resources) {
    handle_timers(resources);
}
