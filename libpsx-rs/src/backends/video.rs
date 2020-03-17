#[cfg(opengl)]
pub mod opengl;

pub enum VideoBackend<'a> {
    None,
    #[cfg(opengl)]
    Opengl(opengl::BackendParams<'a>),
    _Phantom(std::marker::PhantomData<&'a ()>),
}

pub fn setup(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => {},
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::setup(params),
        _ => unimplemented!(),
    }
}

pub fn teardown(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => {},
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::teardown(params),
        _ => unimplemented!(),
    }
}
