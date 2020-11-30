#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::types::{
            rendering::{
                ClutKind,
                TransparencyKind,
            },
        },
        types::ControllerResult,
    },
    types::{
        color::Color,
        geometry::*,
    },
};

pub(crate) fn draw_triangles_shaded(video_backend: &VideoBackend, indices: &[u32], positions: &[Point2D<f32, Normalized>], colors: &[Color], transparency_kind: TransparencyKind) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::draw_triangles_shaded(backend_params, indices, positions, colors, transparency_kind)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_triangles_4_textured(
    video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, TexcoordNormalized>; 4], texture_width: usize, texture_height: usize,
    texture_colors: &[Color],
) -> ControllerResult<Result<(), ()>>
{
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::draw_triangles_4_textured(backend_params, positions, texcoords, texture_width, texture_height, texture_colors)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_triangles_4_textured_framebuffer(
    video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, TexcoordNormalized>; 4], clut_kind: ClutKind,
) -> ControllerResult<Result<(), ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::draw_triangles_4_textured_framebuffer(backend_params, positions, texcoords, clut_kind)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_framebuffer_5551(video_backend: &VideoBackend, origin: Point2D<f32, Normalized>, size: Size2D<f32, Normalized>) -> ControllerResult<Result<Vec<u16>, ()>> {
    match video_backend {
        VideoBackend::None => Ok(Err(())),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(Ok(opengl::read_framebuffer_5551(backend_params, origin, size)?)),
        _ => unimplemented!(),
    }
}
