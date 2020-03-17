#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::backends::video::VideoBackend;

pub(crate) fn render(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => panic!(),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => opengl::render(params),
        _ => unimplemented!(),
    }
}
