#[cfg(openal)]
pub mod openal;

use crate::Config;

pub enum AudioBackend<'a> {
    None,
    #[cfg(openal)]
    Openal(openal::BackendParams<'a>),
    _Phantom(std::marker::PhantomData<&'a ()>),
}

pub(crate) fn setup(config: &Config) {
    match config.audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::setup(config, params),
        _ => unimplemented!(),
    }
}

pub(crate) fn teardown(config: &Config) {
    match config.audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::teardown(config, params),
        _ => unimplemented!(),
    }
}
