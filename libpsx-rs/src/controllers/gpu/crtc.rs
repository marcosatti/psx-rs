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
        handle_vblank_step(resources, duration);
        while handle_vblank_update(resources) {
            handle_vblank(resources, video_backend);
        }
    }
}

fn tick(resources: &mut Resources) {
    handle_timers(resources);
}

fn handle_vblank_step(resources: &mut Resources, duration: Duration) {
    let vblank_time = &mut resources.gpu.crtc.vblank_time;
    *vblank_time += duration;
}

fn handle_vblank_update(resources: &mut Resources) -> bool {
    let vblank_time = &mut resources.gpu.crtc.vblank_time;

    if *vblank_time >= REFRESH_RATE_NTSC_PERIOD {
        *vblank_time -= REFRESH_RATE_NTSC_PERIOD;
        true
    } else {
        false
    }
}
