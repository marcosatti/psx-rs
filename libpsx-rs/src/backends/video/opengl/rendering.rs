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

// TODO: use const generics...
pub(crate) struct ProgramContext {
    pub(crate) program_id: GLuint,
    pub(crate) vao_id: GLuint,
    pub(crate) vbo_ids: Vec<GLuint>,
    pub(crate) texture_ids: Vec<GLuint>,
}

impl ProgramContext {
    pub(crate) fn new(program_id: GLuint, vao_id: GLuint, vbo_ids: &[GLuint], texture_ids: &[GLuint]) -> ProgramContext {
        let mut c = ProgramContext {
            program_id,
            vao_id,
            vbo_ids: Vec::new(),
            texture_ids: Vec::new(),
        };
        c.vbo_ids.extend_from_slice(vbo_ids);
        c.texture_ids.extend_from_slice(texture_ids);
        c
    }
}
