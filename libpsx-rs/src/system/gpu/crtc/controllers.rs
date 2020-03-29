pub mod display;
pub mod backend_dispatch;
pub mod interrupt;

use std::time::Duration;
use crate::video::VideoBackend;
use crate::system::types::State;
use crate::system::gpu::crtc::constants::*;
use crate::system::gpu::crtc::controllers::display::*;
use crate::system::gpu::crtc::controllers::interrupt::*;
use crate::system::gpu::constants::*;

pub fn run_time(state: &mut State, video_backend: &VideoBackend, duration: Duration) {
    state.gpu.crtc.scanline_elapsed += duration;
    while state.gpu.crtc.scanline_elapsed > SCANLINE_INTERVAL_NTSC {
        state.gpu.crtc.scanline_elapsed -= SCANLINE_INTERVAL_NTSC;
        let old_drawing_odd_bit = state.gpu.gpu1814.stat.read_bitfield(STAT_DRAWING_ODD);
        let new_drawing_odd_bit = old_drawing_odd_bit ^ 1;
        state.gpu.gpu1814.stat.write_bitfield(STAT_DRAWING_ODD, new_drawing_odd_bit);
    }

    state.gpu.crtc.frame_elapsed += duration;
    while state.gpu.crtc.frame_elapsed > REFRESH_RATE_NTSC_PERIOD {
        state.gpu.crtc.frame_elapsed -= REFRESH_RATE_NTSC_PERIOD;
        handle_vblank_interrupt(state);
        handle_render(state, video_backend);
    }
}
