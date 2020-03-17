use opengl_sys::*;
use crate::backends::video::opengl;

pub fn render(backend_params: &opengl::BackendParams) {
    static mut PROGRAM_CONTEXT: Option<opengl::rendering::ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        glFinish();

        if PROGRAM_CONTEXT.is_none() {
            let positions_flat: [f32; 8] = [
                -1.0, -1.0,
                1.0, -1.0,
                1.0, 1.0,
                -1.0, 1.0,
            ];

            let texcoords_flat: [f32; 8] = [
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
            ];

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

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: render_opengl");
            }

            PROGRAM_CONTEXT = Some(opengl::rendering::ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);
        glBindVertexArray(program_context.vao_id);

        let mut fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut fbo);

        let mut texture = 0;
        glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut texture);
        
        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, opengl::rendering::WINDOW_FBO);
        
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture as GLuint);

        let tex2d_cstr = b"tex2d\0";
        let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
        glUniform1i(uniform_tex2d, 0);

        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);

        glFinish();

        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo as GLuint);
    }
}
