use sdl2::VideoSubsystem;
use libpsx_rs::backends::video::*;
use libpsx_rs::backends::audio::*;
use libpsx_rs::backends::cdrom::*;

#[cfg(opengl)]
pub(crate) fn initialize_video_backend<'a>(video_subsystem: &'a VideoSubsystem) -> VideoBackend<'a> {
    use sdl2::video::GLProfile;
    use opengl_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(false);
    gl_attr.set_context_flags().debug().set();

    let window = video_subsystem.window("psx-rs", 1024, 512).position_centered().opengl().build().unwrap();
    let opengl_context = window.gl_create_context().unwrap();
    let opengl_acquire_context = || { window.gl_make_current(&opengl_context).unwrap(); &opengl_context };
    let opengl_release_context = || { window.subsystem().gl_release_current_context().unwrap(); };
    opengl_acquire_context();
    let opengl_vendor_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VENDOR as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_version_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VERSION as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_renderer_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_RENDERER as GLenum) as *const i8).to_string_lossy().into_owned() };
    info!("Video initialized: {}, {}, {}", opengl_vendor_string, opengl_version_string, opengl_renderer_string);
    unsafe { glClearColor(0.0, 0.0, 0.0, 1.0); }
    unsafe { glClear(GL_COLOR_BUFFER_BIT); }
    opengl_release_context();

    VideoBackend::Opengl(
        opengl::BackendParams {
            context: BackendContext::new(&opengl_acquire_context, &opengl_release_context),
        }
    )
}

#[cfg(not(opengl))]
pub(crate) fn initialize_video_backend<'a>(_video_subsystem: &'a VideoSubsystem) -> VideoBackend<'a> {
    VideoBackend::None
}

#[cfg(openal)]
pub(crate) fn initialize_audio_backend<'a>() -> AudioBackend<'a> {
    use openal_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    let openal_device = unsafe { alcOpenDevice(std::ptr::null()) };
    let openal_context = unsafe { alcCreateContext(openal_device, std::ptr::null()) };
    let openal_acquire_context = || { unsafe { alcMakeContextCurrent(openal_context); &openal_context } };
    let openal_release_context = || { unsafe { alcMakeContextCurrent(std::ptr::null_mut()); } };
    openal_acquire_context();
    unsafe { alListener3f(AL_POSITION as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListener3f(AL_VELOCITY as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListenerfv(AL_ORIENTATION as ALenum, [0.0, 0.0, -1.0, 0.0, 1.0, 0.0].as_ptr()) };
    let openal_vendor_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VENDOR as ALenum)).to_string_lossy().into_owned() };
    let openal_version_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VERSION as ALenum)).to_string_lossy().into_owned() };
    let openal_renderer_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_RENDERER as ALenum)).to_string_lossy().into_owned() };
    info!("Audio initialized: {}, {}, {}", openal_vendor_string, openal_version_string, openal_renderer_string);
    openal_release_context();

    AudioBackend::Openal(
        openal::BackendParams {
            context: BackendContext::new(&openal_acquire_context, &openal_release_context),
        }
    )
}

#[cfg(not(openal))]
pub(crate) fn initialize_audio_backend<'a>() -> AudioBackend<'a> {
    AudioBackend::None
}

#[cfg(openal)]
pub(crate) fn terminate_audio_backend() {
    use openal_sys::*;
    
    unsafe { alcDestroyContext(openal_context) };
    unsafe { alcCloseDevice(openal_device) };
}

#[cfg(not(openal))]
pub(crate) fn terminate_audio_backend() {
}

#[cfg(libmirage)]
pub(crate) fn initialize_cdrom_backend<'a>() -> CdromBackend<'a> {
    use libmirage_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    unsafe { mirage_initialize(std::ptr::null_mut()) };
    let mut libmirage_context = unsafe { g_object_new(mirage_context_get_type(), std::ptr::null()) as *mut MirageContext };
    let libmirage_acquire_context = || { &libmirage_context };
    let libmirage_release_context = || { };
    let libmirage_version_string = unsafe { std::ffi::CStr::from_ptr(mirage_version_long).to_string_lossy().into_owned() };
    info!("CDROM initialized: libmirage {}", libmirage_version_string);

    CdromBackend::Libmirage(
        libmirage::BackendParams {
            context: BackendContext::new(&libmirage_acquire_context, &libmirage_release_context),
        }
    )
}

#[cfg(not(libmirage))]
pub(crate) fn initialize_cdrom_backend<'a>() -> CdromBackend<'a> {
    CdromBackend::None
}

#[cfg(libmirage)]
pub(crate) fn terminate_cdrom_backend() {
    use libmirage_sys::*;

    unsafe { g_clear_object((&mut libmirage_context as *mut *mut MirageContext) as *mut *mut GObject) };
}

#[cfg(not(libmirage))]
pub(crate) fn terminate_cdrom_backend() {
}
