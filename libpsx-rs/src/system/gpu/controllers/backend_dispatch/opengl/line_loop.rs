use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    system::{
        types::ControllerResult,
        gpu::controllers::backend_dispatch::opengl::{
            MAX_VERTEX_ATTRIBUTE_COUNT,
            debug,
        },
    },
    types::{
        color::*,
        geometry::*,
    },
};
use opengl_sys::*;
use crate::types::array::*;

pub(crate) fn draw_line_loop(backend_params: &BackendParams, positions: &[Point2D<f32, Normalized>]) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());
    assert!(positions.len() <= MAX_VERTEX_ATTRIBUTE_COUNT, "Exceeded max vertex attribute size");

    let [r, g, b, a] = Color::from_8888(255, 0, 0, 255).as_flat();

    let positions_flat = positions.as_flattened();

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let vs = shaders::compile_shader(shaders::vertex::LINE_LOOP, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::LINE_LOOP, GL_FRAGMENT_SHADER);
                let program = shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, MAX_VERTEX_ATTRIBUTE_COUNT as GLsizeiptr * 2 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
            }

            glLineWidth(1.0);

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            let in_color_cstr = b"color\0";
            let in_color_uniform = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(in_color_uniform, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, positions_flat.len() as GLsizeiptr * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_LINE_LOOP, 0, positions.len() as GLint);
        }
    }

    Ok(())
}
