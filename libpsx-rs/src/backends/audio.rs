#[cfg(openal)]
pub mod openal;

#[cfg(any(openal))]
pub enum AudioBackend<'a> {
    None,
    #[cfg(openal)]
    Openal(openal::BackendParams<'a>),
}

#[cfg(any(openal))]
pub enum AudioBackend {
    None,
}

pub fn setup(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::setup(params),
    }
}

pub fn teardown(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::teardown(params),
    }
}
