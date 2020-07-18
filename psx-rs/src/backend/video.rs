#![allow(dead_code)]

use libpsx_rs::backends::video::*;
use sdl2::video::Window;

#[derive(Copy, Clone, Debug)]
pub(crate) enum VideoBackendKind {
    None,
    Opengl,
}

pub(crate) fn initialize_video_backend<'a: 'b, 'b>(kind: VideoBackendKind, window: &'a Window) -> VideoBackend<'a, 'b> {
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
pub(crate) fn initialize_video_backend_opengl<'a: 'b, 'b>(window: &'a Window) -> VideoBackend<'a, 'b> {
    panic!("Not available");
}

#[cfg(opengl)]
pub(crate) fn initialize_video_backend_opengl<'a: 'b, 'b>(window: &'a Window) -> VideoBackend<'a, 'b> {
    use libpsx_rs::backends::context::BackendContext;
    use opengl_sys::*;

    unsafe {
        OPENGL_CONTEXT = Some(window.gl_create_context().unwrap());
        window.gl_make_current(OPENGL_CONTEXT.as_ref().unwrap()).unwrap();
    }

    let opengl_viewport_fn = Box::new(move || {
        let (width, height) = window.drawable_size();
        (width as usize, height as usize)
    });

    let opengl_acquire_context = Box::new(move || {
        unsafe {
            window.gl_make_current(OPENGL_CONTEXT.as_ref().unwrap()).unwrap();
        }
        &()
    });

    let opengl_release_context = Box::new(|| {});

    opengl_acquire_context();
    let opengl_vendor_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VENDOR as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_version_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VERSION as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_renderer_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_RENDERER as GLenum) as *const i8).to_string_lossy().into_owned() };
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);
    }
    log::info!("Video initialized: {}, {}, {}", opengl_vendor_string, opengl_version_string, opengl_renderer_string);
    opengl_release_context();

    VideoBackend::Opengl(opengl::BackendParams {
        context: BackendContext::new(opengl_acquire_context, opengl_release_context),
        callbacks: opengl::Callbacks::new(opengl_viewport_fn),
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
