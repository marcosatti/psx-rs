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

    let drawing_odd = &mut resources.gpu.crtc.drawing_odd;

    *drawing_odd = !*drawing_odd;
    stat.write_bitfield(STAT_DRAWING_ODD, if *drawing_odd { 1 } else { 0 });
    vblank_interrupt(resources);

    render(video_backend);
}

fn vblank_interrupt(resources: &mut Resources) {
    use crate::resources::intc::VBLANK;
    resources.intc.stat.set_irq(VBLANK);
}

fn render(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => { unimplemented!() },
        VideoBackend::Opengl(ref params) => render_opengl(params),
    }
}
