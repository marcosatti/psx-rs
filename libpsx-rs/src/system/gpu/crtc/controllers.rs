pub(crate) mod backend_dispatch;
pub(crate) mod display;
pub(crate) mod interrupt;

use crate::{
    system::{
        gpu::{
            constants::*,
            crtc::{
                constants::*,
                controllers::{
                    display::*,
                    interrupt::*,
                },
            },
        },
        types::{
            ControllerContext,
            ControllerResult,
            Event,
            State,
        },
    },
    video::VideoBackend,
};

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult<()> {
    match event {
        Event::Time(time) => run_time(context.state, context.video_backend, time),
    }
}

pub(crate) fn run_time(state: &State, video_backend: &VideoBackend, duration: f64) -> ControllerResult<()> {
    let crtc_state = &mut state.gpu.crtc.controller_state.lock();

    crtc_state.scanline_elapsed += duration;
    while crtc_state.scanline_elapsed > 0.0 {
        crtc_state.scanline_elapsed -= SCANLINE_NTSC_PERIOD;
        let old_drawing_odd_bit = state.gpu.stat.read_bitfield(STAT_DRAWING_ODD);
        let new_drawing_odd_bit = old_drawing_odd_bit ^ 1;
        state.gpu.stat.write_bitfield(STAT_DRAWING_ODD, new_drawing_odd_bit);
    }

    crtc_state.frame_elapsed += duration;
    while crtc_state.frame_elapsed > 0.0 {
        crtc_state.frame_elapsed -= REFRESH_RATE_NTSC_PERIOD;
        handle_vblank_interrupt(state);
        handle_render(state, video_backend)?;
    }

    Ok(())
}
