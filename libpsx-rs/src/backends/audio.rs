#[cfg(openal)]
pub mod openal;

pub enum AudioBackend<'a: 'b, 'b> {
    None,
    #[cfg(openal)]
    Openal(openal::BackendParams<'a, 'b>),
    _Phantom(std::marker::PhantomData<(&'a (), &'b ())>),
}

pub(crate) fn setup(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::setup(params),
        _ => unimplemented!(),
    }
}

pub(crate) fn teardown(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::None => {},
        #[cfg(openal)]
        AudioBackend::Openal(ref params) => openal::teardown(params),
        _ => unimplemented!(),
    }
}
