use crate::resources::Resources;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::crtc::backend_dispatch;
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
    backend_dispatch::render(video_backend);
}
