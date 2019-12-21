pub mod display;
pub mod opengl;
pub mod interrupt;

use std::time::Duration;
use crate::video::VideoBackend;
use crate::resources::Resources;
use crate::constants::gpu::crtc::*;
use crate::controllers::gpu::crtc::display::*;
use crate::controllers::gpu::crtc::interrupt::*;
use crate::resources::gpu::*;

pub fn run_time(resources: &mut Resources, video_backend: &VideoBackend, duration: Duration) {
    resources.gpu.crtc.scanline_elapsed += duration;
    while resources.gpu.crtc.scanline_elapsed > SCANLINE_INTERVAL_NTSC {
        resources.gpu.crtc.scanline_elapsed -= SCANLINE_INTERVAL_NTSC;
        let old_drawing_odd_bit = resources.gpu.gpu1814.stat.read_bitfield(STAT_DRAWING_ODD);
        let new_drawing_odd_bit = old_drawing_odd_bit ^ 1;
        resources.gpu.gpu1814.stat.write_bitfield(STAT_DRAWING_ODD, new_drawing_odd_bit);
    }

    resources.gpu.crtc.frame_elapsed += duration;
    while resources.gpu.crtc.frame_elapsed > REFRESH_RATE_NTSC_PERIOD {
        resources.gpu.crtc.frame_elapsed -= REFRESH_RATE_NTSC_PERIOD;
        handle_vblank_interrupt(resources);
        handle_render(resources, video_backend);
    }
}
