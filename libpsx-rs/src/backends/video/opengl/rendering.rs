use opengl_sys::*;

pub struct ProgramContext<T: Copy> {
    pub program_id: GLuint,
    pub vao_id: GLuint,
    pub vbo_ids: T,
}

impl<T: Copy> ProgramContext<T> {
    pub fn new(program_id: GLuint, vao_id: GLuint, vbo_ids: T) -> ProgramContext<T> {
        ProgramContext {
            program_id,
            vao_id,
            vbo_ids,
        }
    }
}
