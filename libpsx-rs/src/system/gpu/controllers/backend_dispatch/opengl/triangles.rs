use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    system::{
        gpu::controllers::backend_dispatch::opengl::{
            *,
            debug,
        },
        gpu::{
            controllers::backend_dispatch::opengl::data::*,
            types::{
                rendering::{
                    ClutKind,
                    TransparencyKind,
                },
            },
        },
        types::ControllerResult,
    },
    types::{
        color::*,
        geometry::*,
    },
};
use opengl_sys::*;
use crate::types::array::*;


pub(crate) fn draw_triangles_shaded(
    backend_params: &BackendParams, indices: &[u32], positions: &[Point2D<f32, Normalized>], colors: &[Color], transparency_kind: TransparencyKind,
) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());
    assert!(indices.len() <= MAX_INDICES_COUNT, "Exceeded max indices size");
    assert!(positions.len() <= MAX_VERTEX_ATTRIBUTE_COUNT, "Exceeded max vertex attribute size");
    assert!(colors.len() <= MAX_VERTEX_ATTRIBUTE_COUNT, "Exceeded max vertex attribute size");
    assert!(positions.len() == colors.len(), "Mismatched attribute lengths");

    let positions_flat = positions.as_flattened();
    let colors_flat = colors.as_flattened();
    let transparency_mode_value = transparency_mode_value(transparency_kind);

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let vs = shaders::compile_shader(shaders::vertex::SHADED_TRIANGLES, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::SHADED_TRIANGLES, GL_FRAGMENT_SHADER);
                let program = shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);

                let mut vbo_elements = 0;
                glGenBuffers(1, &mut vbo_elements);
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, vbo_elements);
                glBufferData(GL_ELEMENT_ARRAY_BUFFER, MAX_INDICES_COUNT as GLsizeiptr * std::mem::size_of::<u32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, MAX_VERTEX_ATTRIBUTE_COUNT as GLsizeiptr * 2 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_color = 0;
                glGenBuffers(1, &mut vbo_color);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
                glBufferData(GL_ARRAY_BUFFER, MAX_VERTEX_ATTRIBUTE_COUNT as GLsizeiptr * 4 * std::mem::size_of::<f32>() as GLsizeiptr, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_elements, vbo_position, vbo_color], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            glBindVertexArray(program_context.vao_id);

            let transparency_mode_cstr = b"transparency_mode\0";
            let transparency_mode_uniform = glGetUniformLocation(program_context.program_id, transparency_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(transparency_mode_uniform, transparency_mode_value);
            
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ELEMENT_ARRAY_BUFFER, 0, indices.len() as GLsizeiptr * std::mem::size_of::<u32>() as GLsizeiptr, indices.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, positions_flat.len() as GLsizeiptr * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[2]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, colors_flat.len() as GLsizeiptr * std::mem::size_of::<f32>() as GLsizeiptr, colors_flat.as_ptr() as *const GLvoid);

            glDrawElements(GL_TRIANGLES, indices.len() as GLsizei, GL_UNSIGNED_INT, std::ptr::null());
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop(backend_params, &positions)?;
    }

    Ok(())
}


pub(crate) fn draw_triangles_4_textured(
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
                let vs = shaders::compile_shader(shaders::vertex::TEXTURED_TRIANGLES, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::TEXTURED_TRIANGLES, GL_FRAGMENT_SHADER);
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
                GL_RGBA32F as GLint,
                texture_width as GLsizei,
                texture_height as GLsizei,
                0,
                GL_RGBA,
                GL_FLOAT,
                texture_data_flat.as_ptr() as *const std::ffi::c_void,
            );

            let tex2d_cstr = b"tex2d\0";
            let tex2d_uniform = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(tex2d_uniform, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let clut_mode_uniform = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(clut_mode_uniform, 2);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop(backend_params, &positions)?;
    }

    Ok(())
}

pub(crate) fn draw_triangles_4_textured_framebuffer(
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
                let vs = shaders::compile_shader(shaders::vertex::TEXTURED_TRIANGLES, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::TEXTURED_TRIANGLES, GL_FRAGMENT_SHADER);
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
            let tex2d_uniform = glGetUniformLocation(program_context.program_id, tex2d_cstr.as_ptr() as *const GLchar);
            glUniform1i(tex2d_uniform, 0);

            let clut_mode_cstr = b"clut_mode\0";
            let clut_mode_uniform = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as *const GLchar);
            glUniform1ui(clut_mode_uniform, clut_mode_value);

            let clut_coord_base_cstr = b"clut_coord_base\0";
            let clut_coord_base_uniform = glGetUniformLocation(program_context.program_id, clut_coord_base_cstr.as_ptr() as *const GLchar);
            glUniform2fv(clut_coord_base_uniform, 1, clut_base_value.as_ptr());

            let tex_coord_base_x_cstr = b"tex_coord_base_x\0";
            let tex_coord_base_x_uniform = glGetUniformLocation(program_context.program_id, tex_coord_base_x_cstr.as_ptr() as *const GLchar);
            glUniform1f(tex_coord_base_x_uniform, texcoords[0].x);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, positions_flat.as_ptr() as *const GLvoid);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, 12 * std::mem::size_of::<f32>() as GLsizeiptr, texcoords_flat.as_ptr() as *const GLvoid);

            glBindVertexArray(program_context.vao_id);
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }
    }

    if debug::DEBUG_DRAW_OUTLINE {
        draw_line_loop(backend_params, &positions)?;
    }

    Ok(())
}
