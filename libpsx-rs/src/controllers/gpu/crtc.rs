use std::time::{Duration, Instant};
use opengl_sys::*;
use log::debug;
use crate::constants::gpu::crtc::*;
use crate::backends::video::VideoBackend;
use crate::backends::video::opengl::*;
use crate::State;
use crate::controllers::Event;
use crate::resources::gpu::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    unsafe {
        handle_display(state, duration);
    }
}

unsafe fn handle_display(state: &State, duration: Duration) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    let vblank_time = &mut resources.gpu.crtc.vblank_time;

    let b24_color_depth = stat.read_bitfield(STAT_DISPLAY_COLOR_DEPTH) !=0;
    if b24_color_depth { 
        unimplemented!("24 bit color depth not supported yet"); 
    }

    *vblank_time += duration;
    if *vblank_time >= REFRESH_RATE_NTSC_PERIOD {
        *vblank_time -= REFRESH_RATE_NTSC_PERIOD;
        
        handle_vblank(state);
    }
}

unsafe fn handle_vblank(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    let drawing_odd = &mut resources.gpu.crtc.drawing_odd;

    // Debug: CRTC is way to fast for R3000 (relies on vblank status), so slow it down for now.
    // let instant = Instant::now();
    // while instant.elapsed() < Duration::from_nanos(10_000) {}

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
        VideoBackend::Opengl(ref params) => {
            let (_context_guard, context) = params.context.guard();

            let positions_flat: [f32; 8] = [
                -1.0, -1.0,
                1.0, -1.0,
                1.0, 1.0,
                -1.0, 1.0,
            ];

            let texcoords: [f32; 8] = [
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
            ];

            unsafe {
                let mut fbo = 0;
                glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut fbo);

                let mut texture = 0;
                glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut texture);
                
                glFinish();

                glBindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
                
                glActiveTexture(GL_TEXTURE0);
                glBindTexture(GL_TEXTURE_2D, texture as GLuint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

                let program_id = shaders::get_program(context, shaders::vertex::TEXTURED_POLYGON, shaders::fragment::TEXTURED_POLYGON);
                glUseProgram(program_id);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);

                let tex2d_cstr = b"tex2d\0";
                let uniform_tex2d = glGetUniformLocation(program_id, tex2d_cstr.as_ptr() as *const GLchar);
                glUniform1i(uniform_tex2d, 0);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, (positions_flat.len() * std::mem::size_of::<f32>()) as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, (texcoords.len() * std::mem::size_of::<f32>()) as GLsizeiptr, texcoords.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    
                glBindBuffer(GL_ARRAY_BUFFER, 0);
                glBindVertexArray(0);
                glDisableVertexAttribArray(0);
                glDisableVertexAttribArray(1);
                glDeleteBuffers(1, &mut vbo_position);
                glDeleteBuffers(1, &mut vbo_texcoord);
                glDeleteVertexArrays(1, &mut vao);

                glFinish();

                glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo as GLuint);
            }
        },
    }
}