use crate::State;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::crtc::opengl::*;
use crate::resources::gpu::*;

pub unsafe fn handle_vblank(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) !=0;
    if b24_color_depth { 
        unimplemented!("24 bit color depth not supported yet"); 
    }

    let drawing_odd = &mut resources.gpu.crtc.drawing_odd;

    *drawing_odd = !*drawing_odd;
    stat.write_bitfield(STAT_DRAWING_ODD, if *drawing_odd { 1 } else { 0 });
    vblank_interrupt(state);

    render(state);
}

unsafe fn vblank_interrupt(state: &State) {
    use crate::resources::intc::VBLANK;
    let resources = &mut *state.resources;
    resources.intc.stat.set_irq(VBLANK);
}

fn render(state: &State) {
    match state.video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref params) => render_opengl(params),
    }
}
