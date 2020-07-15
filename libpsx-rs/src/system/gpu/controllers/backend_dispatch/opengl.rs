pub(crate) mod debug {
    pub(crate) const DEBUG_DRAW_OUTLINE: bool = false;
    pub(crate) const TRACE_CALLS: bool = false;

    pub(crate) fn trace_call(description: &str) {
        if TRACE_CALLS {
            log::trace!("GPU: OpenGL call: {}", description);
        }
    }
}

use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    types::{
        color::*,
        geometry::*,
    },
    utilities::array::extract_rectangle,
};
use opengl_sys::*;

pub(crate) fn draw_line_loop_3_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3]) {
    debug::trace_call(stdext::function_name!());

    draw_line_loop_4_solid(backend_params, [positions[0], positions[0], positions[1], positions[2]]);
}

pub(crate) fn draw_line_loop_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r, g, b, a) = Color::new(255, 0, 0, 255).normalize();

    let positions_flat: [f32; 8] = [positions[0].x, positions[0].y, positions[1].x, positions[1].y, positions[3].x, positions[3].y, positions[2].x, positions[2].y];

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let vs = shaders::compile_shader(shaders::vertex::SOLID_LINE_LOOP, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::SOLID_LINE_LOOP, GL_FRAGMENT_SHADER);
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

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
            }

            glLineWidth(1.0);

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            let in_color_cstr = b"in_color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_LINE_LOOP, 0, 4);
        }
    }
}

pub(crate) fn draw_polygon_3_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3], colors: [Color; 3]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r0, g0, b0, a0) = colors[0].normalize();
    let (r1, g1, b1, a1) = colors[1].normalize();
    let (r2, g2, b2, a2) = colors[2].normalize();

    let positions_flat: [f32; 6] = [positions[0].x, positions[0].y, positions[1].x, positions[1].y, positions[2].x, positions[2].y];

    let colors_flat: [f32; 12] = [r0, g0, b0, a0, r1, g1, b1, a1, r2, g2, b2, a2];

    {
        let (_context_guard, _context) = backend_params.context.guard();

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

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_3_solid(backend_params, positions);
    }
}

pub(crate) fn draw_polygon_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], color: Color) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r, g, b, a) = color.normalize();

    let positions_flat: [f32; 12] = [
        positions[0].x,
        positions[0].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[3].x,
        positions[3].y,
    ];

    {
        let (_context_guard, _context) = backend_params.context.guard();

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
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            let in_color_cstr = b"in_color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions);
    }
}

pub(crate) fn draw_polygon_4_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], colors: [Color; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r0, g0, b0, a0) = colors[0].normalize();
    let (r1, g1, b1, a1) = colors[1].normalize();
    let (r2, g2, b2, a2) = colors[2].normalize();
    let (r3, g3, b3, a3) = colors[3].normalize();

    let positions_flat: [f32; 12] = [
        positions[0].x,
        positions[0].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[3].x,
        positions[3].y,
    ];

    let colors_flat: [f32; 24] = [r0, g0, b0, a0, r1, g1, b1, a1, r2, g2, b2, a2, r1, g1, b1, a1, r2, g2, b2, a2, r3, g3, b3, a3];

    {
        let (_context_guard, _context) = backend_params.context.guard();

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
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_color = 0;
                glGenBuffers(1, &mut vbo_color);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
                glBufferData(GL_ARRAY_BUFFER, 24 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_color], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 24 * std::mem::size_of::<f32>() as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions);
    }
}

pub(crate) fn draw_polygon_4_textured(
    backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4], texture_width: usize, texture_height: usize,
    texture_data: &[Color],
)
{
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let positions_flat: [f32; 12] = [
        positions[0].x,
        positions[0].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[3].x,
        positions[3].y,
    ];

    let texcoords_flat: [f32; 12] = [
        texcoords[0].x,
        texcoords[0].y,
        texcoords[1].x,
        texcoords[1].y,
        texcoords[2].x,
        texcoords[2].y,
        texcoords[1].x,
        texcoords[1].y,
        texcoords[2].x,
        texcoords[2].y,
        texcoords[3].x,
        texcoords[3].y,
    ];

    {
        let (_context_guard, _context) = backend_params.context.guard();

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
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut texture = 0;
                glGenTextures(1, &mut texture);
                glActiveTexture(GL_TEXTURE0);
                glBindTexture(GL_TEXTURE_2D, texture);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[texture]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, program_context.texture_ids[0]);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as GLint,
                texture_width as GLsizei,
                texture_height as GLsizei,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                texture_data.as_ptr() as *const std::ffi::c_void,
            );

            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions);
    }
}

pub(crate) fn draw_polygon_4_textured_framebuffer(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, Normalized>; 4]) {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let positions_flat: [f32; 12] = [
        positions[0].x,
        positions[0].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[1].x,
        positions[1].y,
        positions[2].x,
        positions[2].y,
        positions[3].x,
        positions[3].y,
    ];

    let texcoords_flat: [f32; 12] = [
        texcoords[0].x,
        texcoords[0].y,
        texcoords[1].x,
        texcoords[1].y,
        texcoords[2].x,
        texcoords[2].y,
        texcoords[1].x,
        texcoords[1].y,
        texcoords[2].x,
        texcoords[2].y,
        texcoords[3].x,
        texcoords[3].y,
    ];

    {
        let (_context_guard, _context) = backend_params.context.guard();

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
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, 12 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            glActiveTexture(GL_TEXTURE0);
            glReadBuffer(GL_COLOR_ATTACHMENT0);
            glBindTexture(GL_TEXTURE_2D, SCENE_COPY_TEXTURE);
            glCopyTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, 0, 0, SCENE_TEXTURE_WIDTH, SCENE_TEXTURE_HEIGHT, 0);

            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions);
    }
}

pub(crate) fn read_framebuffer_5551(backend_params: &BackendParams, origin: Point2D<f32, Normalized>, size: Size2D<f32, Normalized>) -> Vec<u16> {
    use crate::system::gpu::constants::{
        VRAM_HEIGHT_LINES,
        VRAM_WIDTH_16B,
    };

    debug::trace_call(stdext::function_name!());

    let buffer = unsafe {
        // TODO: non-default framebuffer sizes not handled yet.
        // Could possibly implement by gen'ing another framebuffer of the default size and rendering to that (then extract the
        // texture)?
        assert!(SCENE_TEXTURE_WIDTH as usize == VRAM_WIDTH_16B, "Non-default VRAM size not handled yet");
        assert!(SCENE_TEXTURE_HEIGHT as usize == VRAM_HEIGHT_LINES, "Non-default VRAM size not handled yet");

        let buffer_size = SCENE_TEXTURE_WIDTH as usize * SCENE_TEXTURE_HEIGHT as usize;
        let mut buffer: Vec<u16> = vec![0; buffer_size];

        {
            let (_context_guard, _context) = backend_params.context.guard();

            glFinish();

            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1, buffer.as_mut_ptr() as *mut std::ffi::c_void);
        }

        buffer
    };

    let pixel_origin_x = ((origin.x + 1.0) * ((VRAM_WIDTH_16B / 2) as f32)) as usize;
    let pixel_origin_y = ((origin.y + 1.0) * ((VRAM_HEIGHT_LINES / 2) as f32)) as usize;
    let pixel_origin = Point2D::new(pixel_origin_x, pixel_origin_y);

    let pixel_size_width = ((VRAM_WIDTH_16B / 2) as f32 * size.width) as usize;
    let pixel_size_height = ((VRAM_HEIGHT_LINES / 2) as f32 * size.height) as usize;
    let pixel_size = Size2D::new(pixel_size_width, pixel_size_height);

    extract_rectangle(&buffer, pixel_origin, pixel_size)
}
