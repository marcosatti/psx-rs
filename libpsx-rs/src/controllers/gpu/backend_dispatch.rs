#![allow(unused_variables)]

#[cfg(opengl)]
mod opengl;

use crate::backends::video::VideoBackend;
use crate::types::geometry::{Point2D, Size2D, Normalized, Pixel};
use crate::types::color::Color;

pub(crate) fn draw_polygon_4_solid(video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], color: Color) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::draw_polygon_4_solid(backend_params, positions, color)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_polygon_4_textured_framebuffer(video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4]) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::draw_polygon_4_textured_framebuffer(backend_params, positions, texcoords)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_polygon_3_shaded(video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 3], colors: [Color; 3]) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::draw_polygon_3_shaded(backend_params, positions, colors)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_polygon_4_shaded(video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], colors: [Color; 4]) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::draw_polygon_4_shaded(backend_params, positions, colors)),
        _ => unimplemented!(),
    }
}

pub(crate) fn draw_polygon_4_textured(video_backend: &VideoBackend, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4], texture_width: usize, texture_height: usize, texture_colors: &[Color]) -> Result<(), ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::draw_polygon_4_textured(backend_params, positions, texcoords, texture_width, texture_height, texture_colors)),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_framebuffer_5551(video_backend: &VideoBackend, origin: Point2D<isize, Pixel>, size: Size2D<isize, Pixel>) -> Result<Vec<u16>, ()> {
    match video_backend {
        VideoBackend::None => Err(()),
        #[cfg(opengl)]
        VideoBackend::Opengl(ref backend_params) => Ok(opengl::read_framebuffer_5551(backend_params, origin, size)),
        _ => unimplemented!(),
    }
}
