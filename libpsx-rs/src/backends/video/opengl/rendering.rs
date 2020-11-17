use opengl_sys::*;

pub(crate) static mut WINDOW_FBO: GLuint = 0;
pub(crate) static mut SCENE_FBO: GLuint = 0;
pub(crate) static mut SCENE_TEXTURE: GLuint = 0;
/// Used as a copy of the current scene texture as a source texture when rendering.
/// (Sampling the scene texture whilst also bound to the current FBO is undefined behaviour.)
pub(crate) static mut SCENE_COPY_TEXTURE: GLuint = 0;
pub(crate) static mut SCENE_TEXTURE_WIDTH: GLint = 0;
pub(crate) static mut SCENE_TEXTURE_HEIGHT: GLint = 0;
pub(crate) static mut INTERNAL_SCALE_FACTOR: usize = 1;

pub(crate) struct ProgramContext {
    pub(crate) program_id: GLuint,
    pub(crate) vao_id: GLuint,
    pub(crate) vbo_ids: Box<[GLuint]>,
    pub(crate) texture_ids: Box<[GLuint]>,
}

impl ProgramContext {
    pub(crate) fn new(program_id: GLuint, vao_id: GLuint, vbo_ids: &[GLuint], texture_ids: &[GLuint]) -> ProgramContext {
        ProgramContext {
            program_id,
            vao_id,
            vbo_ids: Box::from(vbo_ids),
            texture_ids: Box::from(texture_ids),
        }
    }
}
