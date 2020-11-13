#![allow(dead_code)]

use libpsx_rs::backends::{
    context::*,
    video::*,
};
use sdl2::video::Window;

#[derive(Copy, Clone, Debug)]
pub(crate) enum VideoBackendKind {
    None,
    Opengl,
}

pub(crate) fn initialize_video_backend<'a>(kind: VideoBackendKind, window: &'a Window) -> VideoBackend<'a> {
    match kind {
        VideoBackendKind::None => VideoBackend::None,
        VideoBackendKind::Opengl => initialize_video_backend_opengl(window),
    }
}

pub(crate) fn terminate_video_backend(kind: VideoBackendKind) {
    match kind {
        VideoBackendKind::None => {},
        VideoBackendKind::Opengl => terminate_video_backend_opengl(),
    }
}

/// Opengl

#[cfg(opengl)]
static mut OPENGL_CONTEXT: Option<sdl2::video::GLContext> = None;

#[cfg(not(opengl))]
pub(crate) fn initialize_video_backend_opengl<'a>(window: &'a Window) -> VideoBackend<'a> {
    panic!("Not available");
}

#[cfg(opengl)]
pub(crate) fn initialize_video_backend_opengl<'a>(window: &'a Window) -> VideoBackend<'a> {
    use opengl_sys::*;

    unsafe {
        OPENGL_CONTEXT = Some(window.gl_create_context().unwrap());
        window.gl_make_current(OPENGL_CONTEXT.as_ref().unwrap()).unwrap();
    }

    let viewport_fn: &'a dyn opengl::Viewport = Box::leak(Box::new(move || {
        let (width, height) = window.drawable_size();
        (width as usize, height as usize)
    }));

    let present_fn: &'a dyn opengl::Present = Box::leak(Box::new(move || {
        window.gl_swap_window();
    }));

    let release_fn = Box::leak(Box::new(move || {
        window.subsystem().gl_release_current_context().unwrap();
    }));

    let acquire_fn = Box::leak(Box::new(move || {
        let context = unsafe { OPENGL_CONTEXT.as_ref().unwrap() };
        window.gl_make_current(context).unwrap();

        opengl::Callbacks {
            viewport_fn,
            present_fn,
        }
    }));

    acquire_fn();
    let opengl_vendor_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VENDOR as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_version_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VERSION as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_renderer_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_RENDERER as GLenum) as *const i8).to_string_lossy().into_owned() };
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);
    }
    present_fn();
    log::info!("Video initialized: {}, {}, {}", opengl_vendor_string, opengl_version_string, opengl_renderer_string);
    release_fn();

    VideoBackend::Opengl(opengl::BackendParams {
        context: BackendContext::new(acquire_fn, release_fn),
    })
}

#[cfg(not(opengl))]
pub(crate) fn terminate_video_backend_opengl() {
    panic!("Not available");
}

#[cfg(opengl)]
pub(crate) fn terminate_video_backend_opengl() {
    unsafe {
        OPENGL_CONTEXT = None;
    }
}
