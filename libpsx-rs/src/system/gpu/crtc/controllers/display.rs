use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            crtc::controllers::backend_dispatch,
        },
        types::{
            ControllerResult,
            State,
        },
    },
};

mod debug {
    use lazy_static::*;
    use parking_lot::Mutex;
    use std::time::{
        Duration,
        Instant,
    };

    const ENABLE_FPS_TRACE: bool = true;
    const FPS_TRACE_REPORT_PERIOD: Duration = Duration::from_secs(1);

    pub(crate) fn trace_fps() {
        static mut FPS_REPORT_INSTANT: Option<Instant> = None;
        static mut FRAME_COUNT: usize = 0;

        lazy_static! {
            static ref AVERAGE: Mutex<(f32, usize)> = Mutex::new((0.0, 0));
        }

        if !ENABLE_FPS_TRACE {
            return;
        }

        unsafe {
            if FPS_REPORT_INSTANT.is_none() {
                FPS_REPORT_INSTANT = Some(Instant::now());
            }

            FRAME_COUNT += 1;

            let elapsed = FPS_REPORT_INSTANT.unwrap().elapsed();

            if elapsed > FPS_TRACE_REPORT_PERIOD {
                let fps = FRAME_COUNT as f32 / elapsed.as_secs_f32();

                let mut average = AVERAGE.lock();
                average.0 += fps;
                average.1 += 1;

                let average = average.0 / average.1 as f32;
                log::trace!("FPS: {:.2} (average: {:.2})", fps, average);

                FPS_REPORT_INSTANT = None;
                FRAME_COUNT = 0;
            }
        }
    }
}

pub(crate) fn handle_render(state: &State, video_backend: &VideoBackend) -> ControllerResult<()> {
    let stat = &state.gpu.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) != 0;
    if b24_color_depth {
        return Err("24 bit color depth not supported yet".into());
    }

    render(video_backend)?;

    Ok(())
}

fn render(video_backend: &VideoBackend) -> ControllerResult<()> {
    let _ = backend_dispatch::render(video_backend)?;

    debug::trace_fps();

    Ok(())
}
