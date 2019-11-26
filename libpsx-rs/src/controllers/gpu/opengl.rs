use opengl_sys::*;
use crate::backends::video::opengl::*;
use crate::backends::video::opengl::rendering::*;
use crate::types::geometry::*;
use crate::types::color::*;
use crate::types::bitfield::*;
use crate::constants::gpu::VRAM_HEIGHT_LINES;

pub fn draw_polygon_3_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3], colors: [Color; 3]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();

    let positions_flat: [f32; 6] = [
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[2].x, positions[2].y,
    ];

    let (r0, g0, b0, a0) = colors[0].normalize();
    let (r1, g1, b1, a1) = colors[1].normalize();
    let (r2, g2, b2, a2) = colors[2].normalize();

    let colors_flat: [f32; 12] = [
        r0, g0, b0, a0,
        r1, g1, b1, a1,
        r2, g2, b2, a2,
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
            glEnableVertexAttribArray(1);

            let mut vbo_position = 0;
            glGenBuffers(1, &mut vbo_position);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
            glBufferData(GL_ARRAY_BUFFER, 6 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            let mut vbo_color = 0;
            glGenBuffers(1, &mut vbo_color);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
            glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: draw_polygon_3_shaded");
            }

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_color], &[]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 6 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid);

        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLES, 0, 3);
    }
}

pub fn draw_polygon_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], color: Color) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();
    
    let positions_flat: [f32; 8] = [
        positions[2].x, positions[2].y,
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
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

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: draw_polygon_4_solid");
            }

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);

        let in_color_cstr = b"in_color\0";
        let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
        glUniform4f(uniform_in_color, r, g, b, a);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn draw_polygon_4_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], colors: [Color; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();

    let positions_flat: [f32; 8] = [
        positions[2].x, positions[2].y,  
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[3].x, positions[3].y,  
    ];

    let (r2, g2, b2, a2) = colors[2].normalize();
    let (r0, g0, b0, a0) = colors[0].normalize();
    let (r1, g1, b1, a1) = colors[1].normalize();
    let (r3, g3, b3, a3) = colors[3].normalize();

    let colors_flat: [f32; 16] = [
        r2, g2, b2, a2,
        r0, g0, b0, a0,
        r1, g1, b1, a1,
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
            glEnableVertexAttribArray(1);

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

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: draw_polygon_4_shaded");
            }

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_color], &[]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 16 * std::mem::size_of::<f32>() as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid);

        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn draw_polygon_4_textured(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4], texture_width: usize, texture_height: usize, texture_data: &[Color]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();

    let positions_flat: [f32; 8] = [
        positions[2].x, positions[2].y,
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[3].x, positions[3].y,    
    ];

    let texcoords_flat: [f32; 8] = [
        texcoords[2].x, texcoords[2].y, 
        texcoords[0].x, texcoords[0].y,
        texcoords[1].x, texcoords[1].y,
        texcoords[3].x, texcoords[3].y,   
    ];

    unsafe {
        if PROGRAM_CONTEXT.is_none() {
            let vs = shaders::compile_shader(shaders::vertex::TEXTURED_POLYGON, GL_VERTEX_SHADER);
            let fs = shaders::compile_shader(shaders::fragment::TEXTURED_POLYGON, GL_FRAGMENT_SHADER);
            let program = shaders::create_program(&[vs, fs]);

            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);

            let mut vbo_position = 0;
            glGenBuffers(1, &mut vbo_position);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            let mut vbo_texcoord = 0;
            glGenBuffers(1, &mut vbo_texcoord);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            let mut texture = 0;
            glGenTextures(1, &mut texture);
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, texture_width as GLsizei, texture_height as GLsizei, 0, GL_RGBA, GL_UNSIGNED_BYTE, std::ptr::null());

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: draw_polygon_4_textured");
            }

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[texture]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, program_context.texture_ids[0]);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, texture_width as GLsizei, texture_height as GLsizei, 0, GL_RGBA, GL_UNSIGNED_BYTE, texture_data.as_ptr() as *const std::ffi::c_void);

        let tex2d_cstr = b"tex2d\0";
        let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
        glUniform1i(uniform_tex2d, 0);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn draw_polygon_4_textured_framebuffer(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    let (_context_guard, _context) = backend_params.context.guard();

    let positions_flat: [f32; 8] = [
        positions[2].x, positions[2].y,
        positions[0].x, positions[0].y,
        positions[1].x, positions[1].y,
        positions[3].x, positions[3].y,    
    ];

    let texcoords_flat: [f32; 8] = [
        texcoords[2].x, texcoords[2].y,
        texcoords[0].x, texcoords[0].y,
        texcoords[1].x, texcoords[1].y,
        texcoords[3].x, texcoords[3].y,    
    ];

    unsafe {
        glFinish();

        if PROGRAM_CONTEXT.is_none() {
            let vs = shaders::compile_shader(shaders::vertex::TEXTURED_POLYGON, GL_VERTEX_SHADER);
            let fs = shaders::compile_shader(shaders::fragment::TEXTURED_POLYGON, GL_FRAGMENT_SHADER);
            let program = shaders::create_program(&[vs, fs]);

            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);

            let mut vbo_position = 0;
            glGenBuffers(1, &mut vbo_position);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            let mut vbo_texcoord = 0;
            glGenBuffers(1, &mut vbo_texcoord);
            glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
            glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
            glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

            if glGetError() != GL_NO_ERROR {
                panic!("Error initializing OpenGL program: draw_polygon_4_textured_framebuffer");
            }

            PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
        }

        let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
        glUseProgram(program_context.program_id);

        let mut texture = 0;
        glGetFramebufferAttachmentParameteriv(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, &mut texture);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture as GLuint);

        let tex2d_cstr = b"tex2d\0";
        let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
        glUniform1i(uniform_tex2d, 0);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

        glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
        glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

        glBindVertexArray(program_context.vao_id);
        glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
    }
}

pub fn read_framebuffer_5551(backend_params: &BackendParams, origin: Point2D<usize, Pixel>, size: Size2D<usize, Pixel>) -> Vec<u16> {
    let (_context_guard, _context) = backend_params.context.guard();
    
    let opengl_origin: Point2D<usize, Pixel> = Point2D::new(
        origin.x,
        VRAM_HEIGHT_LINES - origin.y - size.height,
    );
    
    let mut buffer: Vec<u8> = Vec::new();
    let buffer_size = size.width * size.height * std::mem::size_of::<u16>();
    buffer.resize_with(buffer_size, Default::default);

    unsafe {
        let mut draw_fbo = 0;
        glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut draw_fbo);

        let mut read_fbo = 0;
        glGetIntegerv(GL_READ_FRAMEBUFFER_BINDING, &mut read_fbo);

        glBindBuffer(GL_PIXEL_PACK_BUFFER, 0);

        glBindFramebuffer(GL_READ_FRAMEBUFFER, draw_fbo as GLuint);
        glReadBuffer(GL_COLOR_ATTACHMENT0);
        
        glFinish();

        glReadPixels(opengl_origin.x as GLint, opengl_origin.y as GLint, size.width as GLint, size.height as GLint, GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1, buffer.as_mut_ptr() as *mut std::ffi::c_void);
    
        glBindFramebuffer(GL_READ_FRAMEBUFFER, read_fbo as GLuint);
    }
    
    // Pixel arrangement needs to be from top to bottom, left to right.
    // This means top left corner is (0, 0), and bottom right is (texture_width, texture_height).
    
    let mut fixed_buffer: Vec<u16> = Vec::new();
    let fixed_buffer_size = size.width * size.height;
    fixed_buffer.resize_with(fixed_buffer_size, Default::default);

    for i in 0..fixed_buffer_size {
        let index = (size.width * size.height) - (size.width * (1 + (i / size.width))) + i;
        let mut data: u16 = 0;
        data = Bitfield::new(0, 8).insert_into(data, buffer[index] as u16);
        data = Bitfield::new(8, 8).insert_into(data, buffer[index + 1] as u16);
        fixed_buffer[i] = data;
    }

    fixed_buffer
}
