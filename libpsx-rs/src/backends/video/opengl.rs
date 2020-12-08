pub(crate) mod debug;
pub(crate) mod rendering;
pub(crate) mod shaders;

use crate::{
    backends::context::*,
    system::gpu::constants::{
        VRAM_HEIGHT_LINES,
        VRAM_WIDTH_16B,
    },
    Config,
};
use opengl_sys::*;

pub trait Viewport = Fn() -> (usize, usize);
pub trait Present = Fn() -> ();

#[derive(Copy, Clone)]
pub struct Callbacks<'a> {
    pub viewport_fn: &'a dyn Viewport,
    pub present_fn: &'a dyn Present,
}

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, Callbacks<'a>>,
}

static mut INITIALIZED: bool = false;

pub(crate) fn setup(config: &Config, backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        rendering::INTERNAL_SCALE_FACTOR = config.internal_scale_factor;

        // Debug.
        glEnable(GL_DEBUG_OUTPUT);
        glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);
        glDebugMessageControl(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_TRUE as GLboolean);
        glDebugMessageCallback(Some(debug::debug_callback), std::ptr::null());

        let mut window_fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut window_fbo);

        // Create off-screen FBO. The CRTC controller will handle rendering to the default (window) FBO.
        let mut scene_fbo = 0;
        glGenFramebuffers(1, &mut scene_fbo);
        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, scene_fbo);
        glBindFramebuffer(GL_READ_FRAMEBUFFER, scene_fbo);
        glActiveTexture(GL_TEXTURE0);

        // Create texture for the color attachment.
        let mut scene_texture = 0;
        let scene_texture_width = (VRAM_WIDTH_16B * rendering::INTERNAL_SCALE_FACTOR) as GLint;
        let scene_texture_height = (VRAM_HEIGHT_LINES * rendering::INTERNAL_SCALE_FACTOR) as GLint;
        glGenTextures(1, &mut scene_texture);
        glBindTexture(GL_TEXTURE_2D, scene_texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA32F as GLint, scene_texture_width, scene_texture_height, 0, GL_RGBA, GL_FLOAT, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

        // Attach images to FBO.
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, scene_texture, 0);
        assert!(glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);

        // Save state.
        rendering::WINDOW_FBO = window_fbo as GLuint;
        rendering::SCENE_FBO = scene_fbo as GLuint;
        rendering::SCENE_TEXTURE = scene_texture as GLuint;
        rendering::SCENE_TEXTURE_WIDTH = scene_texture_width;
        rendering::SCENE_TEXTURE_HEIGHT = scene_texture_height;

        // Other.
        glViewport(0, 0, scene_texture_width, scene_texture_height);
        
        glDisable(GL_DITHER);
        glDisable(GL_MULTISAMPLE);

        glPixelStorei(GL_PACK_SWAP_BYTES, GL_FALSE as GLint);
        glPixelStorei(GL_PACK_LSB_FIRST, GL_TRUE as GLint);
        glPixelStorei(GL_PACK_ALIGNMENT, 1);
        glPixelStorei(GL_UNPACK_SWAP_BYTES, GL_FALSE as GLint);
        glPixelStorei(GL_UNPACK_LSB_FIRST, GL_TRUE as GLint);
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);

        glClearColor(0.0, 0.0, 0.0, 0.0);
        glClear(GL_COLOR_BUFFER_BIT);

        if glGetError() != GL_NO_ERROR {
            panic!("Error initializing OpenGL video backend");
        }

        INITIALIZED = true;
    }
}

pub(crate) fn teardown(_config: &Config, backend_params: &BackendParams) {
    // TODO: shader programs are not free'd.

    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            // Delete framebuffer resources and reset back to default.
            glDeleteTextures(1, &rendering::SCENE_TEXTURE);
            glDeleteFramebuffers(1, &rendering::SCENE_FBO);
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, rendering::WINDOW_FBO);
            glBindFramebuffer(GL_READ_FRAMEBUFFER, 0);

            // Debug.
            glDebugMessageCallback(None, std::ptr::null());
            glDebugMessageControl(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_FALSE as GLboolean);

            if glGetError() != GL_NO_ERROR {
                panic!("Error tearing down OpenGL video backend");
            }
        }

        INITIALIZED = false;
    }
}
