pub mod openal;

use crate::backends::audio::openal::*;

pub enum AudioBackend<'a> {
    None,
    Openal(BackendParams<'a>),
}

pub fn setup(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::None => {},
        AudioBackend::Openal(ref params) => openal::setup(params),
    }
}
