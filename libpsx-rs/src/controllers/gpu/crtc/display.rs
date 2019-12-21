use crate::resources::Resources;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::crtc::opengl::*;
use crate::resources::gpu::*;

pub fn handle_render(resources: &Resources, video_backend: &VideoBackend) {
    let stat = &resources.gpu.gpu1814.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) != 0;
    if b24_color_depth { 
        unimplemented!("24 bit color depth not supported yet"); 
    }

    render(video_backend);
}

fn render(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => { unimplemented!() },
        VideoBackend::Opengl(ref params) => render_opengl(params),
    }
}
