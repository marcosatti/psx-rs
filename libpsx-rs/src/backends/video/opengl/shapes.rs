use opengl_sys::*;
use crate::backends::video::opengl::shaders::*;

pub struct Polygon4 {
    program: GLuint,
    vao: GLuint,
    vertex_positions: GLuint,
    vertex_colors: GLuint,

}