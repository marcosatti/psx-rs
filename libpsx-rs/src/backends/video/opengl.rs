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

        let mut window_fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut window_fbo);

        // Create off-screen FBO. The CRTC controller will handle rendering to the default (window) FBO.
        let mut scene_fbo = 0;
        glGenFramebuffers(1, &mut scene_fbo);
        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, scene_fbo);

        // Create texture for the color attachment.
        let mut color_texture = 0;
        glGenTextures(1, &mut color_texture);
        glBindTexture(GL_TEXTURE_2D, color_texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, VRAM_WIDTH_16B as GLint, VRAM_HEIGHT_LINES as GLint, 0, GL_RGBA, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

        // Attach images to FBO.
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, color_texture, 0);
        assert!(glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);

        // Save state.
        rendering::WINDOW_FBO = window_fbo as GLuint;
        rendering::SCENE_FBO = scene_fbo as GLuint;
        rendering::SCENE_TEXTURE = color_texture as GLuint;

        // Other.
        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);

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
            // Delete framebuffer resources and reset back to default.
            glDeleteTextures(1, &rendering::SCENE_TEXTURE);
            glDeleteFramebuffers(1, &rendering::SCENE_FBO);
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
