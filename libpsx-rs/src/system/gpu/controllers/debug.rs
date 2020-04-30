use crate::{
    system::{
        gpu::crtc::controllers::display::handle_render,
        types::State,
    },
    video::VideoBackend,
};
use std::{
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    thread::sleep,
    time::Duration,
};

pub static ENABLE_GP0_COMMAND_TRACING: AtomicBool = AtomicBool::new(false);
pub static ENABLE_GP0_RENDER_PER_CALL: AtomicBool = AtomicBool::new(false);

pub fn trace_gp0_command(description: &str, data: &[u32]) {
    if !ENABLE_GP0_COMMAND_TRACING.load(Ordering::Acquire) {
        return;
    }

    let data_str = data.iter().map(|d| format!("0x{:08X}", d)).collect::<Vec<String>>().join(", ");
    log::trace!("GP0 Comamnd: {}: data = [{}]", description, &data_str);
}

pub fn trace_gp0_command_render(state: &State, video_backend: &VideoBackend) {
    if !ENABLE_GP0_RENDER_PER_CALL.load(Ordering::Acquire) {
        return;
    }

    handle_render(state, video_backend);
    let duration = Duration::from_millis(200);
    log::trace!("Draw call issued; render performed (sleeping {} ms)", duration.as_millis());
    sleep(duration);
}
