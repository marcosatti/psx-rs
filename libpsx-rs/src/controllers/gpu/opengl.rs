use opengl_sys::*;
use crate::backends::video::opengl::*;
use crate::backends::video::opengl::rendering::*;
use crate::types::geometry::*;
use crate::types::color::*;

pub fn draw_polygon_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], color: Color) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext<(GLuint,)>> = None;

    let (_gl_context_guard, context) = backend_params.context.guard();
    
    let positions_flat: [f32; 8] = [
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[2].x, positions[2].y,
        positions[3].x, positions[3].y,  
    ];

    let (r, g, b, a) = color.normalize();

    unsafe {
        if PROGRAM_CONTEXT.is_none() {
            let vs = shaders::compile_shader(shaders::vertex::SOLID_POLYGON, GL_VERTEX_SHADER);
            let fs = shaders::compile_shader(shaders::fragment::SOLID_POLYGON, GL_FRAGMENT_SHADER);
            let program = shaders::create_program(&[vs, fs]);

            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
            glEnableVertexAttribArray(0);

            let mut vbo_position = 0;
            glGenBuffers(1, &mut vbo_position);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, (vbo_position,)));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();

        let in_color_cstr = b"in_color\0";
        let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
        glUniform4f(uniform_in_color, r, g, b, a);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids.0);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glUseProgram(program_context.program_id);
        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn draw_polygon_4_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], colors: [Color; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext<(GLuint, GLuint)>> = None;

    let (_gl_context_guard, context) = backend_params.context.guard();

    let positions_flat: [f32; 8] = [
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[2].x, positions[2].y,
        positions[3].x, positions[3].y,    
    ];

    let (r0, g0, b0, a0) = colors[0].normalize();
    let (r1, g1, b1, a1) = colors[1].normalize();
    let (r2, g2, b2, a2) = colors[2].normalize();
    let (r3, g3, b3, a3) = colors[3].normalize();

    let colors_flat: [f32; 16] = [
        r0, g0, b0, a0,
        r1, g1, b1, a1,
        r2, g2, b2, a2,
        r3, g3, b3, a3,
    ];

    unsafe {
        if PROGRAM_CONTEXT.is_none() {
            let vs = shaders::compile_shader(shaders::vertex::SHADED_POLYGON, GL_VERTEX_SHADER);
            let fs = shaders::compile_shader(shaders::fragment::SHADED_POLYGON, GL_FRAGMENT_SHADER);
            let program = shaders::create_program(&[vs, fs]);

            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
            glEnableVertexAttribArray(0);

            let mut vbo_position = 0;
            glGenBuffers(1, &mut vbo_position);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            let mut vbo_color = 0;
            glGenBuffers(1, &mut vbo_color);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
            glBufferData(GL_ARRAY_BUFFER, 16 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, (vbo_position, vbo_color)));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids.0);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids.1);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 16 * std::mem::size_of::<f32>() as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid);

        glUseProgram(program_context.program_id);
        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn draw_polygon_3_shaded(backend_params: &BackendParams, positions: [Point2D<f32>; 3], colors: [(u8, u8, u8, u8); 3]) {
    let (_gl_context_guard, context) = backend_params.context.guard();

    let positions_flat: [f32; 6] = [
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[2].x, positions[2].y,
    ];

    let colors_flat: [u8; 12] = [
        colors[0].0, colors[0].1, colors[0].2, colors[0].3,
        colors[1].0, colors[1].1, colors[1].2, colors[1].3,
        colors[2].0, colors[2].1, colors[2].2, colors[2].3,
    ];

    unsafe {
        let program_id = shaders::get_program(context, shaders::vertex::SHADED_POLYGON, shaders::fragment::SHADED_POLYGON);
        glUseProgram(program_id);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

        let mut vbo_position = 0;
        glGenBuffers(1, &mut vbo_position);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
        glBufferData(GL_ARRAY_BUFFER, (positions_flat.len() * std::mem::size_of::<f32>()) as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

        let mut vbo_color = 0;
        glGenBuffers(1, &mut vbo_color);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
        glBufferData(GL_ARRAY_BUFFER, (colors_flat.len() * std::mem::size_of::<u8>()) as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(1, 4, GL_UNSIGNED_BYTE, GL_TRUE as GLboolean, 0, std::ptr::null());

        glDrawArrays(GL_TRIANGLES, 0, 3);

        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindVertexArray(0);
        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);
        glDeleteBuffers(1, &mut vbo_position);
        glDeleteBuffers(1, &mut vbo_color);
        glDeleteVertexArrays(1, &mut vao);
    }
}

pub fn draw_rectangle_textured(backend_params: &BackendParams, rect: Rect<f32>, texture_width: usize, texture_height: usize, texture_data: &[u8]) {
    let (_gl_context_guard, context) = backend_params.context.guard();

    // Triangle fan, 4 vertices drawn counter clockwise.

    let positions_flat: [f32; 8] = [
        rect.min_x(), rect.min_y(),
        rect.max_x(), rect.min_y(),
        rect.max_x(), rect.max_y(),
        rect.min_x(), rect.max_y(),
    ];

    // Texture data comes through as left to right, top to bottom - need to flip it about the x-axis (see opengl texcoords space).
    let texcoords: [f32; 8] = [
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 0.0,
    ];

    unsafe {
        let program_id = shaders::get_program(context, shaders::vertex::TEXTURED_POLYGON, shaders::fragment::TEXTURED_POLYGON);
        glUseProgram(program_id);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

        let mut texture = 0;
        glGenTextures(1, &mut texture);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, texture_width as GLsizei, texture_height as GLsizei, 0, GL_RGB, GL_UNSIGNED_BYTE, texture_data.as_ptr() as *const std::ffi::c_void);
        let tex2d_cstr = b"tex2d\0";
        let uniform_tex2d = glGetUniformLocation(program_id, tex2d_cstr.as_ptr() as *const GLchar);
        glUniform1i(uniform_tex2d, 0);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

        let mut vbo_position = 0;
        glGenBuffers(1, &mut vbo_position);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
        glBufferData(GL_ARRAY_BUFFER, (positions_flat.len() * std::mem::size_of::<f32>()) as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

        let mut vbo_texcoord = 0;
        glGenBuffers(1, &mut vbo_texcoord);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
        glBufferData(GL_ARRAY_BUFFER, (texcoords.len() * std::mem::size_of::<f32>()) as GLsizeiptr, texcoords.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);

        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindVertexArray(0);
        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);
        glDeleteTextures(1, &mut texture);
        glDeleteBuffers(1, &mut vbo_position);
        glDeleteBuffers(1, &mut vbo_texcoord);
        glDeleteVertexArrays(1, &mut vao);
    }
}

pub fn draw_polygon_4_fb_blended(backend_params: &BackendParams, positions: [Point2D<f32>; 4], texcoords: [Point2D<f32>; 4]) {
    let (_gl_context_guard, context) = backend_params.context.guard();

    let positions_flat: [f32; 8] = [
        positions[2].x, positions[2].y,
        positions[3].x, positions[3].y,     
        positions[1].x, positions[1].y,
        positions[0].x, positions[0].y,
    ];

    let texcoords: [f32; 8] = [
        texcoords[2].x, texcoords[2].y,
        texcoords[3].x, texcoords[3].y,
        texcoords[1].x, texcoords[1].y,
        texcoords[0].x, texcoords[0].y,
    ];

    unsafe {
        let program_id = shaders::get_program(context, shaders::vertex::TEXTURED_POLYGON, shaders::fragment::TEXTURED_POLYGON);
        glUseProgram(program_id);

        glFinish();
        let mut fbo = 0;
        glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut fbo);
        let mut texture = 0;
        glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut texture);
        let tex2d_cstr = b"tex2d\0";
        let uniform_tex2d = glGetUniformLocation(program_id, tex2d_cstr.as_ptr() as *const GLchar);
        glUniform1i(uniform_tex2d, 0);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture as GLuint);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

        let mut vbo_position = 0;
        glGenBuffers(1, &mut vbo_position);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
        glBufferData(GL_ARRAY_BUFFER, (positions_flat.len() * std::mem::size_of::<f32>()) as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

        let mut vbo_texcoord = 0;
        glGenBuffers(1, &mut vbo_texcoord);
        glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
        glBufferData(GL_ARRAY_BUFFER, (texcoords.len() * std::mem::size_of::<f32>()) as GLsizeiptr, texcoords.as_ptr() as *const GLvoid, GL_STATIC_DRAW);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);

        glBindTexture(GL_TEXTURE_2D, 0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindVertexArray(0);
        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);
        glDeleteBuffers(1, &mut vbo_position);
        glDeleteBuffers(1, &mut vbo_texcoord);
        glDeleteVertexArrays(1, &mut vao);
    }
}

pub fn read_framebuffer(backend_params: &BackendParams) -> Vec<u16> {
    let (_gl_context_guard, _context) = backend_params.context.guard();

    let mut data_rgb16 = vec![0_u16; 1024 * 512];

    unsafe {
        glFlush();
        let mut fbo = 0;
        glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut fbo);
        let mut texture = 0;
        glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut texture);
        glFinish();

        glBindTexture(GL_TEXTURE_2D, texture as GLuint);
        glGetTexImage(GL_TEXTURE_2D, 0, GL_RGB, GL_UNSIGNED_SHORT_5_5_5_1, data_rgb16.as_mut_ptr() as *mut std::ffi::c_void);
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    data_rgb16
}