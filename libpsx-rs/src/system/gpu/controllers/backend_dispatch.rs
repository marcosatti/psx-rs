#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::types::rendering::*,
        types::ControllerResult,
    },
    types::color::*,
};

pub(crate) fn read_framebuffer(video_backend: &VideoBackend, params: ReadFramebufferParams) -> ControllerResult<Result<Vec<PackedColor>, ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::read_framebuffer(backend_params, params)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn write_framebuffer(video_backend: &VideoBackend, params: WriteFramebufferParams) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::write_framebuffer(backend_params, params)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_rectangle(video_backend: &VideoBackend, params: RectangleParams) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::draw_rectangle(backend_params, params)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_triangles(video_backend: &VideoBackend, params: TrianglesParams) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::draw_triangles(backend_params, params)?)),
        _ => unimplemented!(),
    }
}
