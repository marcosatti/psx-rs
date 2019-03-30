use opengl_sys::*;

// TODO: use const generics...
pub struct ProgramContext {
    pub program_id: GLuint,
    pub vao_id: GLuint,
    pub vbo_ids: Vec<GLuint>,
    pub texture_ids: Vec<GLuint>,
}

impl<T: Copy> ProgramContext<T> {
    pub fn new(program_id: GLuint, vao_id: GLuint, vbo_ids: &[GLuint], texture_ids: &[GLuint]) -> ProgramContext<T> {
        let mut c = ProgramContext {
            program_id: program_id,
            vao_id: vao_id,
            vbo_ids: Vec::new(),
            texture_ids: Vec::new(),
        }
        c.vbo_ids.extend_from_slice(vbo_ids);
        c.texture_ids.extend_from_slice(texture_ids);
        c
    }
}
