use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    system::{
        gpu::controllers::backend_dispatch::opengl::{
            debug,
        },
        gpu::{
            controllers::backend_dispatch::opengl::data::*,
            types::{
                rendering::{
                    *,
                },
            },
        },
        types::ControllerResult,
    },
};
use opengl_sys::*;
use crate::types::array::*;
use crate::utilities::bool_to_flag;

pub(crate) fn draw_triangles(backend_params: &BackendParams, params: TrianglesParams) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());
    assert!(params.vertices == params.positions.len());
    assert!(params.vertices == params.colors.len());
    assert!(params.vertices == params.texture_position_offsets.len());

    let indices_len = match params.vertices {
        3 => 3,
        4 => 6,
        _ => panic!("Unsupported number of vertices: {}", params.vertices),
    };

    let positions_flat = make_positions_normalized(params.positions).as_flattened();
    let colors_flat = make_colors_normalized(params.colors).as_flattened();
    let texture_position_offsets_flat = make_texture_position_offsets_normalized(params.texture_position_offsets).as_flattened();
    let rendering_mode_value = rendering_mode_value(params.rendering_kind);
    let texture_position_base_value = texture_position_base_value(params.rendering_kind);
    let clut_mode_value = clut_mode_value(params.rendering_kind);
    let clut_texture_position_base_value = clut_texture_position_base_value(params.rendering_kind);
    let transparency_mode_value = transparency_mode_value(params.transparency_kind);
    let mask_bit_force_set_value = bool_to_flag(params.mask_bit_force_set) as i32;
    let mask_bit_check_value = bool_to_flag(params.mask_bit_check) as i32;

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let indices: [u32; 6] = [0, 1, 2, 1, 2, 3];

                let vs = shaders::compile_shader(shaders::vertex::TRIANGLES, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::TRIANGLES, GL_FRAGMENT_SHADER);
                let program = shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);
                glEnableVertexAttribArray(2);

                let mut vbo_elements = 0;
                glGenBuffers(1, &mut vbo_elements);
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, vbo_elements);
                glBufferData(GL_ELEMENT_ARRAY_BUFFER, (6 * std::mem::size_of::<u32>()) as _, indices.as_ptr() as _, GL_STATIC_DRAW);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                let mut vbo_color = 0;
                glGenBuffers(1, &mut vbo_color);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_color);
                glBufferData(GL_ARRAY_BUFFER, (12 * std::mem::size_of::<f32>()) as _, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());
                
                let mut vbo_texture_position_offset = 0;
                glGenBuffers(1, &mut vbo_texture_position_offset);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texture_position_offset);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(2, 2, GL_FLOAT, GL_FALSE as GLboolean, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_elements, vbo_position, vbo_color, vbo_texture_position_offset], &[]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);

            glBindVertexArray(program_context.vao_id);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);

            let framebuffer_cstr = b"framebuffer\0";
            let framebuffer_uniform = glGetUniformLocation(program_context.program_id, framebuffer_cstr.as_ptr() as _);
            glUniform1i(framebuffer_uniform, 0);
            
            let rendering_mode_cstr = b"rendering_mode\0";
            let rendering_mode_uniform = glGetUniformLocation(program_context.program_id, rendering_mode_cstr.as_ptr() as _);
            glUniform1ui(rendering_mode_uniform, rendering_mode_value);

            let texture_position_base_cstr = b"texture_position_base\0";
            let texture_position_base_uniform = glGetUniformLocation(program_context.program_id, texture_position_base_cstr.as_ptr() as _);
            glUniform2fv(texture_position_base_uniform, 1, texture_position_base_value.as_ptr());
            
            let clut_mode_cstr = b"clut_mode\0";
            let clut_mode_uniform = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as _);
            glUniform1ui(clut_mode_uniform, clut_mode_value);

            let clut_texture_position_base_cstr = b"clut_texture_position_base\0";
            let clut_texture_position_base_uniform = glGetUniformLocation(program_context.program_id, clut_texture_position_base_cstr.as_ptr() as _);
            glUniform2fv(clut_texture_position_base_uniform, 1, clut_texture_position_base_value.as_ptr());
            
            let transparency_mode_cstr = b"transparency_mode\0";
            let transparency_mode_uniform = glGetUniformLocation(program_context.program_id, transparency_mode_cstr.as_ptr() as _);
            glUniform1ui(transparency_mode_uniform, transparency_mode_value);

            let mask_bit_force_set_cstr = b"mask_bit_force_set\0";
            let mask_bit_force_set_uniform = glGetUniformLocation(program_context.program_id, mask_bit_force_set_cstr.as_ptr() as _);
            glUniform1i(mask_bit_force_set_uniform, mask_bit_force_set_value);

            let mask_bit_check_cstr = b"mask_bit_check\0";
            let mask_bit_check_uniform = glGetUniformLocation(program_context.program_id, mask_bit_check_cstr.as_ptr() as _);
            glUniform1i(mask_bit_check_uniform, mask_bit_check_value);
            
            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[1]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, (params.vertices * 2 * std::mem::size_of::<f32>()) as _, positions_flat.as_ptr() as _);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[2]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, (params.vertices * 3 * std::mem::size_of::<f32>()) as _, colors_flat.as_ptr() as _);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[3]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, (params.vertices * 2 * std::mem::size_of::<f32>()) as _, texture_position_offsets_flat.as_ptr() as _);

            glTextureBarrier();
            glDrawElements(GL_TRIANGLES, indices_len, GL_UNSIGNED_INT, std::ptr::null());
        }
    }

    Ok(())
}
