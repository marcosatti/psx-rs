use crate::{
    system::{
        gpu::crtc::controllers::display::handle_render,
        types::State,
    },
    video::VideoBackend,
};
use std::{
    thread::sleep,
    time::Duration,
};

const ENABLE_GP0_COMMAND_TRACING: bool = false;
const ENABLE_GP0_RENDER_PER_CALL: bool = false;

pub(crate) fn trace_gp0_command(description: &str, data: &[u32]) {
    if !ENABLE_GP0_COMMAND_TRACING {
        return;
    }

    let data_str = data.iter().map(|d| format!("0x{:08X}", d)).collect::<Vec<String>>().join(", ");
    log::trace!("GP0 Comamnd: {}: data = [{}]", description, &data_str);
}

pub(crate) fn trace_gp0_command_render(state: &State, video_backend: &VideoBackend) {
    if !ENABLE_GP0_RENDER_PER_CALL {
        return;
    }

    handle_render(state, video_backend);
    let duration = Duration::from_millis(200);
    log::trace!("Draw call issued; render performed (sleeping {} ms)", duration.as_millis());
    sleep(duration);
}
