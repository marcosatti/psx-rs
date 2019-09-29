pub mod opengl;

use crate::backends::video::opengl::*;

pub enum VideoBackend<'a> {
    None,
    Opengl(BackendParams<'a>),
}
