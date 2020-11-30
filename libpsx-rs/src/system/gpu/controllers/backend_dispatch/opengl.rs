pub(crate) mod data;
pub(crate) mod debug;
pub(crate) mod line_loop;
pub(crate) mod framebuffer;
pub(crate) mod triangles;

pub(crate) use line_loop::*;
pub(crate) use framebuffer::*;
pub(crate) use triangles::*;

const MAX_VERTEX_ATTRIBUTE_COUNT: usize = 16;
const MAX_INDICES_COUNT: usize = 32;
