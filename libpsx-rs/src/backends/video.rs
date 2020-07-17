#[cfg(opengl)]
pub mod opengl;

use crate::Config;

pub enum VideoBackend<'a: 'b, 'b> {
    None,
    #[cfg(opengl)]
    Opengl(opengl::BackendParams<'a, 'b>),
    _Phantom(std::marker::PhantomData<(&'a (), &'b ())>),
}

pub(crate) fn setup(config: &Config) {
    match config.video_backend {
        VideoBackend::None => {},
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::setup(config, params),
        _ => unimplemented!(),
    }
}

pub(crate) fn teardown(config: &Config) {
    match config.video_backend {
        VideoBackend::None => {},
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::teardown(config, params),
        _ => unimplemented!(),
    }
}
