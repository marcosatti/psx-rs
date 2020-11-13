#![allow(dead_code)]

use libpsx_rs::backends::audio::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum AudioBackendKind {
    None,
    Openal,
}

pub(crate) fn initialize_audio_backend<'a>(kind: AudioBackendKind) -> AudioBackend<'a> {
    match kind {
        AudioBackendKind::None => AudioBackend::None,
        AudioBackendKind::Openal => initialize_audio_backend_openal(),
    }
}

pub(crate) fn terminate_audio_backend(kind: AudioBackendKind) {
    match kind {
        AudioBackendKind::None => {},
        AudioBackendKind::Openal => terminate_audio_backend_openal(),
    }
}

// Openal

#[cfg(openal)]
static mut OPENAL_DEVICE: *mut openal_sys::ALCdevice = std::ptr::null_mut();

#[cfg(openal)]
static mut OPENAL_CONTEXT: *mut openal_sys::ALCcontext = std::ptr::null_mut();

#[cfg(not(openal))]
pub(crate) fn initialize_audio_backend_openal<'a>() -> AudioBackend<'a> {
    panic!("Not available");
}

#[cfg(openal)]
pub(crate) fn initialize_audio_backend_openal<'a>() -> AudioBackend<'a> {
    use libpsx_rs::backends::context::BackendContext;
    use openal_sys::*;

    unsafe {
        OPENAL_DEVICE = alcOpenDevice(std::ptr::null());
        assert!(!OPENAL_DEVICE.is_null());
        OPENAL_CONTEXT = alcCreateContext(OPENAL_DEVICE, std::ptr::null());
        assert!(!OPENAL_CONTEXT.is_null());
        alcMakeContextCurrent(OPENAL_CONTEXT);
    }

    unsafe { alListener3f(AL_POSITION as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListener3f(AL_VELOCITY as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListenerfv(AL_ORIENTATION as ALenum, [0.0, 0.0, -1.0, 0.0, 1.0, 0.0].as_ptr()) };
    let openal_vendor_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VENDOR as ALenum)).to_string_lossy().into_owned() };
    let openal_version_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VERSION as ALenum)).to_string_lossy().into_owned() };
    let openal_renderer_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_RENDERER as ALenum)).to_string_lossy().into_owned() };
    log::info!("Audio initialized: {}, {}, {}", openal_vendor_string, openal_version_string, openal_renderer_string);

    AudioBackend::Openal(openal::BackendParams {
        context: BackendContext::new(&move || {}, &move || {}),
    })
}

#[cfg(not(openal))]
pub(crate) fn terminate_audio_backend_openal() {
    panic!("Not available");
}

#[cfg(openal)]
pub(crate) fn terminate_audio_backend_openal() {
    use openal_sys::*;

    unsafe {
        assert!(!OPENAL_CONTEXT.is_null());
        alcDestroyContext(OPENAL_CONTEXT);
        assert!(!OPENAL_DEVICE.is_null());
        alcCloseDevice(OPENAL_DEVICE);
    }
}
