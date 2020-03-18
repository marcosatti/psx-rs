#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::backends::video::VideoBackend;

pub(crate) fn render(video_backend: &VideoBackend) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => Ok(opengl::render(params)),
        _ => unimplemented!(),
    }
}
