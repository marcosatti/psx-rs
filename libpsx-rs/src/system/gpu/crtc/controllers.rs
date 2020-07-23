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
    let controller_state = &mut state.gpu.crtc.controller_state.lock();
    controller_state.scanline_clock += duration;
    controller_state.frame_clock += duration;

    loop {
        let mut handled = false;

        if controller_state.scanline_clock > 0.0 {
            handle_scanline_tick(state);
            controller_state.scanline_clock -= SCANLINE_NTSC_PERIOD;
            handled = true;
        }

        if controller_state.frame_clock > 0.0 {
            handle_frame_tick(state, video_backend)?;
            controller_state.frame_clock -= REFRESH_RATE_NTSC_PERIOD;
            handled = true;
        }

        if !handled {
            break;
        }
    }

    Ok(())
}

fn handle_scanline_tick(state: &State) {
    let drawing_odd_bit = state.gpu.stat.read_bitfield(STAT_DRAWING_ODD) ^ 1;
    state.gpu.stat.write_bitfield(STAT_DRAWING_ODD, drawing_odd_bit);
}

fn handle_frame_tick(state: &State, video_backend: &VideoBackend) -> ControllerResult<()> {
    handle_vblank_interrupt(state);
    handle_render(state, video_backend)?;

    Ok(())
}
