use libpsx_rs::backends::{
    audio::*,
    cdrom::*,
    video::*,
};
use sdl2::video::Window;

/// Video

#[cfg(opengl)]
static mut OPENGL_CONTEXT: Option<sdl2::video::GLContext> = None;

#[cfg(opengl)]
pub(crate) fn initialize_video_backend<'a: 'b, 'b>(window: &'a Window) -> VideoBackend<'a, 'b> {
    use libpsx_rs::backends::context::BackendContext;
    use opengl_sys::*;

    unsafe {
        OPENGL_CONTEXT = Some(window.gl_create_context().unwrap());
        window.gl_make_current(OPENGL_CONTEXT.as_ref().unwrap()).unwrap();
    }

    // TODO: need to consider multithreading? It's a bit unclear, but doesn't look like it - probably implementation
    // dependant...
    let opengl_acquire_context = move || {
        unsafe {
            window.gl_make_current(OPENGL_CONTEXT.as_ref().unwrap()).unwrap();
        }
        &()
    };
    let opengl_release_context = || {};

    opengl_acquire_context();
    let opengl_vendor_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VENDOR as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_version_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VERSION as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_renderer_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_RENDERER as GLenum) as *const i8).to_string_lossy().into_owned() };
    unsafe {
        glClearColor(0.0, 0.0, 0.0, 1.0);
    }
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT);
    }
    log::info!("Video initialized: {}, {}, {}", opengl_vendor_string, opengl_version_string, opengl_renderer_string);
    opengl_release_context();

    VideoBackend::Opengl(opengl::BackendParams {
        context: BackendContext::new(Box::new(opengl_acquire_context), Box::new(opengl_release_context)),
    })
}

#[cfg(not(opengl))]
pub(crate) fn initialize_video_backend<'a: 'b, 'b>(_window: &'a Window) -> VideoBackend<'a, 'b> {
    VideoBackend::None
}

#[cfg(opengl)]
pub(crate) fn terminate_video_backend() {
    unsafe {
        OPENGL_CONTEXT = None;
    }
}

#[cfg(not(opengl))]
pub(crate) fn terminate_video_backend() {
}

/// Audio

#[cfg(openal)]
const MUTED: bool = true;

#[cfg(openal)]
static mut OPENAL_DEVICE: *mut openal_sys::ALCdevice = std::ptr::null_mut();

#[cfg(openal)]
static mut OPENAL_CONTEXT: *mut openal_sys::ALCcontext = std::ptr::null_mut();

#[cfg(openal)]
pub(crate) fn initialize_audio_backend<'a: 'b, 'b>() -> AudioBackend<'a, 'b> {
    if !MUTED {
        use libpsx_rs::backends::context::BackendContext;
        use openal_sys::*;

        unsafe {
            OPENAL_DEVICE = alcOpenDevice(std::ptr::null());
            assert!(!OPENAL_DEVICE.is_null());
            OPENAL_CONTEXT = alcCreateContext(OPENAL_DEVICE, std::ptr::null());
            assert!(!OPENAL_CONTEXT.is_null());
            alcMakeContextCurrent(OPENAL_CONTEXT);
        }

        // TODO: need to consider multithreading? It's a bit unclear, but doesn't look like it - probably implementation
        // dependant...
        let openal_acquire_context = || &();
        let openal_release_context = || {};

        openal_acquire_context();
        unsafe { alListener3f(AL_POSITION as ALenum, 0.0, 0.0, 0.0) };
        unsafe { alListener3f(AL_VELOCITY as ALenum, 0.0, 0.0, 0.0) };
        unsafe { alListenerfv(AL_ORIENTATION as ALenum, [0.0, 0.0, -1.0, 0.0, 1.0, 0.0].as_ptr()) };
        let openal_vendor_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VENDOR as ALenum)).to_string_lossy().into_owned() };
        let openal_version_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VERSION as ALenum)).to_string_lossy().into_owned() };
        let openal_renderer_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_RENDERER as ALenum)).to_string_lossy().into_owned() };
        log::info!("Audio initialized: {}, {}, {}", openal_vendor_string, openal_version_string, openal_renderer_string);
        openal_release_context();

        AudioBackend::Openal(openal::BackendParams {
            context: BackendContext::new(Box::new(openal_acquire_context), Box::new(openal_release_context)),
        })
    } else {
        AudioBackend::None
    }
}

#[cfg(not(openal))]
pub(crate) fn initialize_audio_backend<'a: 'b, 'b>() -> AudioBackend<'a, 'b> {
    AudioBackend::None
}

#[cfg(openal)]
pub(crate) fn terminate_audio_backend() {
    if !MUTED {
        use openal_sys::*;

        unsafe {
            assert!(!OPENAL_CONTEXT.is_null());
            alcDestroyContext(OPENAL_CONTEXT);
            assert!(!OPENAL_DEVICE.is_null());
            alcCloseDevice(OPENAL_DEVICE);
        }
    } else {
    }
}

#[cfg(not(openal))]
pub(crate) fn terminate_audio_backend() {
}

/// CDROM

#[cfg(libmirage)]
static mut LIBMIRAGE_CONTEXT: *mut libmirage_sys::MirageContext = std::ptr::null_mut();

#[cfg(libmirage)]
pub(crate) fn initialize_cdrom_backend<'a: 'b, 'b>() -> CdromBackend<'a, 'b> {
    use libmirage_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    unsafe {
        mirage_initialize(std::ptr::null_mut());
        LIBMIRAGE_CONTEXT = g_object_new(mirage_context_get_type(), std::ptr::null()) as *mut MirageContext;
        assert!(!LIBMIRAGE_CONTEXT.is_null());
    }

    let libmirage_acquire_context = || unsafe { &LIBMIRAGE_CONTEXT };
    let libmirage_release_context = || {};
    let libmirage_version_string = unsafe { std::ffi::CStr::from_ptr(mirage_version_long).to_string_lossy().into_owned() };
    log::info!("CDROM initialized: libmirage {}", libmirage_version_string);

    CdromBackend::Libmirage(libmirage::BackendParams {
        context: BackendContext::new(Box::new(libmirage_acquire_context), Box::new(libmirage_release_context)),
    })
}

#[cfg(not(libmirage))]
pub(crate) fn initialize_cdrom_backend<'a: 'b, 'b>() -> CdromBackend<'a, 'b> {
    CdromBackend::None
}

#[cfg(libmirage)]
pub(crate) fn terminate_cdrom_backend() {
    use libmirage_sys::*;

    unsafe {
        assert!(!LIBMIRAGE_CONTEXT.is_null());
        g_clear_object((&mut LIBMIRAGE_CONTEXT as *mut *mut MirageContext) as *mut *mut GObject);
    }
}

#[cfg(not(libmirage))]
pub(crate) fn terminate_cdrom_backend() {
}
