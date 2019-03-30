pub mod shaders;
pub mod rendering;

use crate::backends::context::*;

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, sdl2::video::GLContext>,
}
