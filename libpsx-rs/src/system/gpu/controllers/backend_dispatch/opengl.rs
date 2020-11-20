pub(crate) mod data;
pub(crate) mod debug;

use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    system::{
        gpu::{
            controllers::backend_dispatch::opengl::data::*,
            types::{
                rendering::ClutKind,
                TransparencyMode,
            },
        },
        types::ControllerResult,
    },
    types::{
        color::*,
        geometry::*,
    },
    utilities::array::extract_rectangle,
};
use opengl_sys::*;

pub(crate) fn draw_line_loop_3_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3]) -> ControllerResult<()> {
    debug::trace_call(stdext::function_name!());

    draw_line_loop_4_solid(backend_params, [positions[0], positions[0], positions[1], positions[2]])?;

    Ok(())
}

pub(crate) fn draw_line_loop_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4]) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r, g, b, a) = Color::from_8888(255, 0, 0, 255).as_flat();

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

            let in_color_cstr = b"color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_LINE_LOOP, 0, 4);
        }
    }

    Ok(())
}

pub(crate) fn draw_polygon_3_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3], colors: [Color; 3]) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r0, g0, b0, a0) = colors[0].as_flat();
    let (r1, g1, b1, a1) = colors[1].as_flat();
    let (r2, g2, b2, a2) = colors[2].as_flat();

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
        draw_line_loop_3_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_4_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], color: Color) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r, g, b, a) = color.as_flat();

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

            let in_color_cstr = b"color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_4_transparent(
    backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], color: Color, transparency: TransparencyMode,
) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    log::warn!("Transparency not implemented in shaders yet");

    let (r, g, b, a) = color.as_flat();

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

    let transparency_value = transparency_value(transparency);

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

            let in_color_cstr = b"color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            let transparency_cstr = b"transparency\0";
            let uniform_transparency = glGetUniformLocation(program_context.program_id, transparency_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_transparency, transparency_value);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_3_solid(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3], color: Color) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r, g, b, a) = color.as_flat();

    let positions_flat: [f32; 6] = [positions[0].x, positions[0].y, positions[1].x, positions[1].y, positions[2].x, positions[2].y];

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
                glBufferData(GL_ARRAY_BUFFER, 6 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            let in_color_cstr = b"color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 6 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_3_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_3_transparent(
    backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 3], color: Color, transparency: TransparencyMode,
) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    log::warn!("Transparency not implemented in shaders yet");

    let (r, g, b, a) = color.as_flat();

    let positions_flat: [f32; 6] = [positions[0].x, positions[0].y, positions[1].x, positions[1].y, positions[2].x, positions[2].y];

    let transparency_value = transparency_value(transparency);

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
                glBufferData(GL_ARRAY_BUFFER, 6 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            let in_color_cstr = b"color\0";
            let uniform_in_color = glGetUniformLocation(program_context.program_id, in_color_cstr.as_ptr() as *const GLchar);
            glUniform4f(uniform_in_color, r, g, b, a);

            let transparency_cstr = b"transparency\0";
            let uniform_transparency = glGetUniformLocation(program_context.program_id, transparency_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_transparency, transparency_value);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 6 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_3_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_4_shaded(backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], colors: [Color; 4]) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let (r0, g0, b0, a0) = colors[0].as_flat();
    let (r1, g1, b1, a1) = colors[1].as_flat();
    let (r2, g2, b2, a2) = colors[2].as_flat();
    let (r3, g3, b3, a3) = colors[3].as_flat();

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
        draw_line_loop_4_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_4_textured(
    backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, TexcoordNormalized>; 4], texture_width: usize, texture_height: usize,
    texture_data: &[Color],
) -> ControllerResult<()>
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

    let mut texture_data_flat: Vec<f32> = vec![0.0; texture_data.len() * 4];
    for (i, color) in texture_data.iter().enumerate() {
        texture_data_flat[(i * 4) + 0] = color.r;
        texture_data_flat[(i * 4) + 1] = color.g;
        texture_data_flat[(i * 4) + 2] = color.b;
        texture_data_flat[(i * 4) + 3] = color.a;
    }

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
                glBindTexture(GL_TEXTURE_2D, texture);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[texture]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            glBindTexture(GL_TEXTURE_2D, program_context.texture_ids[0]);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as GLint,
                texture_width as GLsizei,
                texture_height as GLsizei,
                0,
                GL_RGBA,
                GL_FLOAT,
                texture_data_flat.as_ptr() as *const std::ffi::c_void,
            );

            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let uniform_clut_mode = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_clut_mode, 2);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn draw_polygon_4_textured_framebuffer(
    backend_params: &BackendParams, positions: [Point2D<f32, Normalized>; 4], texcoords: [Point2D<f32, TexcoordNormalized>; 4], clut_kind: ClutKind,
) -> ControllerResult<()> {
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

    let clut_mode_value = clut_mode_value(clut_kind);

    let clut_base_value = clut_base_value(clut_kind);

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

            glReadBuffer(GL_COLOR_ATTACHMENT0);
            glBindTexture(GL_TEXTURE_2D, SCENE_COPY_TEXTURE);
            glCopyTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, 0, 0, SCENE_TEXTURE_WIDTH, SCENE_TEXTURE_HEIGHT, 0);

            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let uniform_clut_mode = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_clut_mode, clut_mode_value);

            let clut_coord_base_cstr = b"clut_coord_base\0";
            let uniform_clut_coord_base = glGetUniformLocation(program_context.program_id, clut_coord_base_cstr.as_ptr() as *const GLchar);
            glUniform2fv(uniform_clut_coord_base, 1, clut_base_value.as_ptr());

            let tex_coord_base_x_cstr = b"tex_coord_base_x\0";
            let uniform_tex_coord_base_x = glGetUniformLocation(program_context.program_id, tex_coord_base_x_cstr.as_ptr() as *const GLchar);
            glUniform1f(uniform_tex_coord_base_x, texcoords[0].x);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop_4_solid(backend_params, positions)?;
    }

    Ok(())
}

pub(crate) fn read_framebuffer_5551(backend_params: &BackendParams, origin: Point2D<f32, Normalized>, size: Size2D<f32, Normalized>) -> ControllerResult<Vec<u16>> {
    use crate::system::gpu::constants::{
        VRAM_HEIGHT_LINES,
        VRAM_WIDTH_16B,
    };

    // TODO: resources are not free'd.
    static mut FBO_CONTEXT: Option<(GLuint, GLuint)> = None;
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let downsample_texture_width = VRAM_WIDTH_16B as GLint;
    let downsample_texture_height = VRAM_HEIGHT_LINES as GLint;

    // Setup a temporary FBO with 1x internal resolution - we need this to extract the exact pixels out of.
    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            glFinish();

            if FBO_CONTEXT.is_none() {
                let mut downsample_fbo = 0;
                glGenFramebuffers(1, &mut downsample_fbo);
                glBindFramebuffer(GL_DRAW_FRAMEBUFFER, downsample_fbo);

                let mut downsample_texture = 0;
                glGenTextures(1, &mut downsample_texture);
                glBindTexture(GL_TEXTURE_2D, downsample_texture);
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, downsample_texture_width, downsample_texture_height, 0, GL_RGBA, GL_UNSIGNED_BYTE, std::ptr::null());
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLint);

                glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, downsample_texture, 0);
                assert!(glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);

                FBO_CONTEXT = Some((downsample_fbo, downsample_texture));
            }

            if PROGRAM_CONTEXT.is_none() {
                let positions_flat: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0];

                let texcoords_flat: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];

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
                glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const std::ffi::c_void, GL_STATIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, 8 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const std::ffi::c_void, GL_STATIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
            }

            let fbo_context = FBO_CONTEXT.unwrap();
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo_context.0);
            glViewport(0, 0, downsample_texture_width, downsample_texture_height);

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            glBindVertexArray(program_context.vao_id);

            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);
            let tex2d_cstr = b"tex2d\0";
            let uniform_tex2d = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(uniform_tex2d, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let uniform_clut_mode = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(uniform_clut_mode, 2);

            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
            glFinish();

            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, SCENE_FBO);
            glViewport(0, 0, SCENE_TEXTURE_WIDTH, SCENE_TEXTURE_HEIGHT);
        }
    }

    let mut buffer: Vec<u16> = vec![0; VRAM_WIDTH_16B * VRAM_HEIGHT_LINES];

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            let fbo_context = FBO_CONTEXT.unwrap();
            glBindTexture(GL_TEXTURE_2D, fbo_context.1);
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1, buffer.as_mut_ptr() as *mut std::ffi::c_void);
        }
    }

    let pixel_origin_x = ((origin.x + 1.0) * ((VRAM_WIDTH_16B / 2) as f32)) as usize;
    let pixel_origin_y = ((origin.y + 1.0) * ((VRAM_HEIGHT_LINES / 2) as f32)) as usize;
    let pixel_origin = Point2D::new(pixel_origin_x, pixel_origin_y);

    let pixel_size_width = ((VRAM_WIDTH_16B / 2) as f32 * size.width) as usize;
    let pixel_size_height = ((VRAM_HEIGHT_LINES / 2) as f32 * size.height) as usize;
    let pixel_size = Size2D::new(pixel_size_width, pixel_size_height);

    Ok(extract_rectangle(&buffer, pixel_origin, pixel_size))
}
