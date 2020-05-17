use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            crtc::controllers::backend_dispatch,
        },
        types::State,
    },
};

pub(crate) fn handle_render(state: &State, video_backend: &VideoBackend) {
    let stat = &state.gpu.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) != 0;
    if b24_color_depth {
        unimplemented!("24 bit color depth not supported yet");
    }

    render(video_backend);
}

fn render(video_backend: &VideoBackend) {
    let _ = backend_dispatch::render(video_backend);
}
