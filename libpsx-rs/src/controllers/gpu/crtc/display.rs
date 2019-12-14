//use log::debug;
use crate::resources::Resources;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::crtc::opengl::*;
use crate::resources::gpu::*;

pub fn handle_vblank(resources: &mut Resources, video_backend: &VideoBackend) {
    let stat = &mut resources.gpu.gpu1814.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) !=0;
    if b24_color_depth { 
        unimplemented!("24 bit color depth not supported yet"); 
    }

    render(video_backend);
    vblank_interrupt(resources);
}

fn vblank_interrupt(resources: &mut Resources) {
    use crate::resources::intc::VBLANK;
    resources.intc.stat.set_irq(VBLANK);
    //debug!("VBLANK interrupt fired");
}

fn render(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => { unimplemented!() },
        VideoBackend::Opengl(ref params) => render_opengl(params),
    }
}
