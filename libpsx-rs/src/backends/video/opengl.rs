pub mod shaders;
pub mod rendering;
pub mod debug;

use opengl_sys::*;
use crate::backends::context::*;
use crate::constants::gpu::{VRAM_WIDTH_16B, VRAM_HEIGHT_LINES}; 

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, sdl2::video::GLContext>,
}

pub fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        glDebugMessageControlARB(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_TRUE as GLboolean);
        glDebugMessageCallbackARB(Some(debug::debug_callback), std::ptr::null());

        let mut window_fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut window_fbo);
        rendering::WINDOW_FBO = window_fbo as GLuint;

        let mut fbo = 0;
        glGenFramebuffers(1, &mut fbo);
        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo);

        let mut texture = 0;
        glGenTextures(1, &mut texture);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, 0, GL_RGB, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);  

        let mut rbo = 0;
        glGenRenderbuffers(1, &mut rbo);
        glBindRenderbuffer(GL_RENDERBUFFER, rbo);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint);
        
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, texture, 0);
        glFramebufferRenderbuffer(GL_DRAW_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, rbo); 

        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        if glGetError() != GL_NO_ERROR {
            panic!("Error initializing OpenGL video backend");
        }
    }
}
