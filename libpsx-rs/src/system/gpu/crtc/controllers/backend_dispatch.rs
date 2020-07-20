#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::{
    backends::video::VideoBackend,
    system::types::ControllerResult,
};

pub(crate) fn render(video_backend: &VideoBackend) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref params) => Ok(Ok(opengl::render(params)?)),
        _ => unimplemented!(),
    }
}
