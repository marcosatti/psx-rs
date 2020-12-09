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
use crate::utilities::bool_to_flag;

pub(crate) fn draw_rectangle(backend_params: &BackendParams, params: RectangleParams) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let positions_flat = make_triangle_fan(params.rectangle);
    let position_base_value = normalize_position(params.rectangle.origin).to_array();
    let color_value = params.color.to_normalized().as_flat();
    let rendering_mode_value = rendering_mode_value(params.rendering_kind);
    let texture_position_base_value = texture_position_base_value(params.rendering_kind);
    let texture_position_base_offset_value = normalize_texture_size(params.texture_position_base_offset).to_array();
    let clut_mode_value = clut_mode_value(params.rendering_kind);
    let clut_texture_position_base_value = clut_texture_position_base_value(params.rendering_kind);
    let transparency_mode_value = transparency_mode_value(params.transparency_kind);
    let drawing_area_top_left_flat = normalize_position(params.drawing_area.min()).to_array();
    let drawing_area_bottom_right_flat = normalize_position(params.drawing_area.max()).to_array();
    let mask_bit_force_set_value = bool_to_flag(params.mask_bit_force_set) as i32;
    let mask_bit_check_value = bool_to_flag(params.mask_bit_check) as i32;

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let vs = shaders::compile_shader(shaders::vertex::RECTANGLE, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::RECTANGLE, GL_FRAGMENT_SHADER);
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

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            
            glBindVertexArray(program_context.vao_id);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);

            let framebuffer_cstr = b"framebuffer\0";
            let framebuffer_uniform = glGetUniformLocation(program_context.program_id, framebuffer_cstr.as_ptr() as _);
            glUniform1i(framebuffer_uniform, 0);

            let position_base_cstr = b"position_base\0";
            let position_base_uniform = glGetUniformLocation(program_context.program_id, position_base_cstr.as_ptr() as _);
            glUniform2fv(position_base_uniform, 1, position_base_value.as_ptr());

            let color_cstr = b"color\0";
            let color_uniform = glGetUniformLocation(program_context.program_id, color_cstr.as_ptr() as _);
            glUniform3fv(color_uniform, 1, color_value.as_ptr());

            let rendering_mode_cstr = b"rendering_mode\0";
            let rendering_mode_uniform = glGetUniformLocation(program_context.program_id, rendering_mode_cstr.as_ptr() as _);
            glUniform1ui(rendering_mode_uniform, rendering_mode_value);

            let texture_position_base_cstr = b"texture_position_base\0";
            let texture_position_base_uniform = glGetUniformLocation(program_context.program_id, texture_position_base_cstr.as_ptr() as _);
            glUniform2fv(texture_position_base_uniform, 1, texture_position_base_value.as_ptr());
            
            let texture_position_base_offset_cstr = b"texture_position_base_offset\0";
            let texture_position_base_offset_uniform = glGetUniformLocation(program_context.program_id, texture_position_base_offset_cstr.as_ptr() as _);
            glUniform2fv(texture_position_base_offset_uniform, 1, texture_position_base_offset_value.as_ptr());
            
            let clut_mode_cstr = b"clut_mode\0";
            let clut_mode_uniform = glGetUniformLocation(program_context.program_id, clut_mode_cstr.as_ptr() as _);
            glUniform1ui(clut_mode_uniform, clut_mode_value);

            let clut_texture_position_base_cstr = b"clut_texture_position_base\0";
            let clut_texture_position_base_uniform = glGetUniformLocation(program_context.program_id, clut_texture_position_base_cstr.as_ptr() as _);
            glUniform2fv(clut_texture_position_base_uniform, 1, clut_texture_position_base_value.as_ptr());
            
            let transparency_mode_cstr = b"transparency_mode\0";
            let transparency_mode_uniform = glGetUniformLocation(program_context.program_id, transparency_mode_cstr.as_ptr() as _);
            glUniform1ui(transparency_mode_uniform, transparency_mode_value);
            
            let drawing_area_top_left_cstr = b"drawing_area_top_left\0";
            let drawing_area_top_left_uniform = glGetUniformLocation(program_context.program_id, drawing_area_top_left_cstr.as_ptr() as _);
            glUniform2fv(drawing_area_top_left_uniform, 1, drawing_area_top_left_flat.as_ptr());

            let drawing_area_bottom_right_cstr = b"drawing_area_bottom_right\0";
            let drawing_area_bottom_right_uniform = glGetUniformLocation(program_context.program_id, drawing_area_bottom_right_cstr.as_ptr() as _);
            glUniform2fv(drawing_area_bottom_right_uniform, 1, drawing_area_bottom_right_flat.as_ptr());

            let mask_bit_force_set_cstr = b"mask_bit_force_set\0";
            let mask_bit_force_set_uniform = glGetUniformLocation(program_context.program_id, mask_bit_force_set_cstr.as_ptr() as _);
            glUniform1i(mask_bit_force_set_uniform, mask_bit_force_set_value);

            let mask_bit_check_cstr = b"mask_bit_check\0";
            let mask_bit_check_uniform = glGetUniformLocation(program_context.program_id, mask_bit_check_cstr.as_ptr() as _);
            glUniform1i(mask_bit_check_uniform,mask_bit_check_value);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, (8 * std::mem::size_of::<f32>()) as _, positions_flat.as_ptr() as _);

            glTextureBarrier();
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
        }
    }

    Ok(())
}
