pub mod opengl;

use crate::backends::video::opengl::*;

pub enum VideoBackend<'a> {
    Opengl(BackendParams<'a>),
}
