pub mod display;
pub mod opengl;

use std::time::Duration;
use crate::video::VideoBackend;
use crate::resources::Resources;
use crate::constants::gpu::crtc::*;
use crate::controllers::gpu::crtc::display::*;

pub fn run_time(resources: &mut Resources, video_backend: &VideoBackend, duration: Duration) {
    handle_vblank_step(resources, duration);
    while handle_vblank_update(resources) {
        handle_vblank(resources, video_backend);
    }
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
