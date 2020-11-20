pub(crate) mod debug {
    pub(crate) const TRACE_CALLS: bool = false;

    pub(crate) fn trace_call(description: &str) {
        if TRACE_CALLS {
            log::trace!("GPU CRTC: OpenGL call: {}", description);
        }
    }
}

use crate::{
    backends::video::opengl,
    system::types::ControllerResult,
};
use opengl_sys::*;

pub(crate) fn render(backend_params: &opengl::BackendParams) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<opengl::rendering::ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    {
        let (_context_guard, context) = backend_params.context.guard();
        let (width, height) = (context.viewport_fn)();

        unsafe {
            glFinish();

            if PROGRAM_CONTEXT.is_none() {
                let positions_flat: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0];

                let texcoords_flat: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];

                let vs = opengl::shaders::compile_shader(opengl::shaders::vertex::TEXTURED_POLYGON, GL_VERTEX_SHADER);
                let fs = opengl::shaders::compile_shader(opengl::shaders::fragment::TEXTURED_POLYGON, GL_FRAGMENT_SHADER);
                let program = opengl::shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const std::ffi::c_void, GL_STATIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const std::ffi::c_void, GL_STATIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(opengl::rendering::ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            glBindVertexArray(program_context.vao_id);

            // Bind the window FBO so it's now active.
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, opengl::rendering::WINDOW_FBO);

            // Set the required viewport.
            glViewport(0, 0, width as GLint, height as GLint);

            // Bind the off-screen texture to the uniform variable.
            // Don't need to use the copy texture here as we are using a different FBO to render to.
            glBindTexture(GL_TEXTURE_2D, opengl::rendering::SCENE_TEXTURE);
            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let uniform_clut_mode = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_clut_mode, 2);

            // Draw the off-screen texture to the window FBO, hard synchronise after.
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
            glFinish();

            (context.present_fn)();

            // Bind the off-screen FBO again, ready for the GPU to draw into.
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, opengl::rendering::SCENE_FBO);

            // Reset viewport for off-screen rendering.
            glViewport(0, 0, opengl::rendering::SCENE_TEXTURE_WIDTH, opengl::rendering::SCENE_TEXTURE_HEIGHT);
        }
    }

    Ok(())
}
