pub mod open_gl;

use crate::backends::video::open_gl::*;

pub enum VideoBackend<'a> {
    OpenGl(BackendParams<'a>),
}
