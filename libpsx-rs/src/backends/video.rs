#[cfg(opengl)]
pub mod opengl;

#[cfg(any(opengl)]
pub enum VideoBackend<'a> {
    None,
    #[cfg(opengl)]
    Opengl(opengl::BackendParams<'a>),
}

#[cfg(not(any(opengl)))]
pub enum VideoBackend {
    None,
}

pub fn setup(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => unimplemented!(),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::setup(params),
    }
}

pub fn teardown(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => unimplemented!(),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::teardown(params),
    }
}
