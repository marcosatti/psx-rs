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

static mut INITIALIZED: bool = false;

type ViewportCallbackFn<'a> = Box<dyn Fn() -> (usize, usize) + 'a>;

pub struct Callbacks<'a> {
    pub(crate) viewport_callback_fn: ViewportCallbackFn<'a>,
}

impl<'a> Callbacks<'a> {
    pub fn new(viewport_callback_fn: ViewportCallbackFn<'a>) -> Callbacks<'a> {
        Callbacks {
            viewport_callback_fn,
        }
    }
}

unsafe impl<'a> Send for Callbacks<'a> {
}

unsafe impl<'a> Sync for Callbacks<'a> {
}

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, ()>,
    pub callbacks: Callbacks<'a>,
}

pub(crate) fn setup(config: &Config, backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        rendering::INTERNAL_SCALE_FACTOR = config.internal_scale_factor;

        // Debug.
        glDebugMessageControlARB(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_TRUE as GLboolean);
        glDebugMessageCallbackARB(Some(debug::debug_callback), std::ptr::null());

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
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, scene_texture_width, scene_texture_height, 0, GL_RGBA, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);

        // Attach images to FBO.
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, scene_texture, 0);
        assert!(glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);

        // Create scene copy texture.
        let mut scene_copy_texture = 0;
        glGenTextures(1, &mut scene_copy_texture);
        glBindTexture(GL_TEXTURE_2D, scene_copy_texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, scene_texture_width, scene_texture_height, 0, GL_RGBA, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);

        // Save state.
        rendering::WINDOW_FBO = window_fbo as GLuint;
        rendering::SCENE_FBO = scene_fbo as GLuint;
        rendering::SCENE_TEXTURE = scene_texture as GLuint;
        rendering::SCENE_COPY_TEXTURE = scene_copy_texture as GLuint;
        rendering::SCENE_TEXTURE_WIDTH = scene_texture_width;
        rendering::SCENE_TEXTURE_HEIGHT = scene_texture_height;

        // Other.
        glViewport(0, 0, scene_texture_width, scene_texture_height);
        glClearColor(0.0, 0.0, 0.0, 1.0);
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
            glDeleteTextures(1, &rendering::SCENE_COPY_TEXTURE);
            glDeleteFramebuffers(1, &rendering::SCENE_FBO);
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, rendering::WINDOW_FBO);
            glBindFramebuffer(GL_READ_FRAMEBUFFER, 0);

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
