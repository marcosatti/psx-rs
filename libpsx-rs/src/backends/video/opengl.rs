pub(crate) mod debug;
pub(crate) mod rendering;
pub(crate) mod shaders;

use crate::{
    backends::context::*,
    system::gpu::constants::{
        VRAM_HEIGHT_LINES,
        VRAM_WIDTH_16B,
    },
};
use opengl_sys::*;

static mut INITIALIZED: bool = false;

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, ()>,
}

pub(crate) fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        // Debug.
        glDebugMessageControlARB(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_TRUE as GLboolean);
        glDebugMessageCallbackARB(Some(debug::debug_callback), std::ptr::null());

        // Create off-screen FBO. The CRTC controller will handle rendering to the default (window) FBO.
        let mut window_fbo = 0;
        let mut fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut window_fbo);
        rendering::WINDOW_FBO = window_fbo as GLuint;
        glGenFramebuffers(1, &mut fbo);
        glBindFramebuffer(GL_FRAMEBUFFER, fbo);

        // Create texture for the color attachment.
        let mut color_texture = 0;
        glGenTextures(1, &mut color_texture);
        glBindTexture(GL_TEXTURE_2D, color_texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, 0, GL_RGB, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

        // Create texture for stencil and depth.
        let mut depth_stencil_texture = 0;
        glGenTextures(1, &mut depth_stencil_texture);
        glBindTexture(GL_TEXTURE_2D, depth_stencil_texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_DEPTH_STENCIL as GLint, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, 0, GL_DEPTH_STENCIL, GL_FLOAT, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

        // Attach texutres to FBO.
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, color_texture, 0);
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_TEXTURE_2D, depth_stencil_texture, 0);

        // Other.
        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClearDepth(0.0);
        glClearStencil(0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);

        if glGetError() != GL_NO_ERROR {
            panic!("Error initializing OpenGL video backend");
        }

        INITIALIZED = true;
    }
}

pub(crate) fn teardown(backend_params: &BackendParams) {
    // TODO: shader programs are not free'd.

    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            // Delete color texture.
            let mut param = 0;
            glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, &mut param);
            assert!(param == (GL_TEXTURE as GLint));
            glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut param);
            let texture = param as GLuint;
            glDeleteTextures(1, &texture);

            // Delete depth & stencil texture.
            let mut param = 0;
            glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE, &mut param);
            assert!(param == (GL_TEXTURE as GLint));
            glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut param);
            let texture = param as GLuint;
            glDeleteTextures(1, &texture);

            // Delete FBO.
            let mut fbo_int = 0;
            glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut fbo_int);
            let fbo = fbo_int as GLuint;
            glDeleteFramebuffers(1, &fbo);
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, rendering::WINDOW_FBO);

            // Debug.
            glDebugMessageCallbackARB(None, std::ptr::null());
            glDebugMessageControlARB(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_FALSE as GLboolean);

            if glGetError() != GL_NO_ERROR {
                panic!("Error tearing down OpenGL video backend");
            }
        }

        INITIALIZED = false;
    }
}
