use crate::{
    backends::video::opengl::{
        rendering::*,
        *,
    },
    system::{
        gpu::{
            controllers::backend_dispatch::opengl::{
                debug,
                data::*,
            },
            types::rendering::*,
        },
        types::ControllerResult,
    },
    types::{
        color::*,
    },
    utilities::array::*,
};
use crate::system::gpu::constants::{
    VRAM_HEIGHT_LINES,
    VRAM_WIDTH_16B,
};
use opengl_sys::*;
use crate::utilities::bool_to_flag;

pub(crate) fn read_framebuffer(backend_params: &BackendParams, params: ReadFramebufferParams) -> ControllerResult<Vec<PackedColor>> {
    // TODO: resources are not free'd.
    static mut FBO_CONTEXT: Option<(GLuint, GLuint)> = None;
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());
    
    let mut rect = params.rectangle;
    rect.origin.y -= rect.size.height;

    let downsample_texture_width = VRAM_WIDTH_16B as GLint;
    let downsample_texture_height = VRAM_HEIGHT_LINES as GLint;

    // Setup a temporary FBO with 1x internal resolution - we need this to extract the exact pixels out of.
    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if FBO_CONTEXT.is_none() {
                let mut downsample_fbo = 0;
                glGenFramebuffers(1, &mut downsample_fbo);
                glBindFramebuffer(GL_DRAW_FRAMEBUFFER, downsample_fbo);

                let mut downsample_texture = 0;
                glGenTextures(1, &mut downsample_texture);
                glBindTexture(GL_TEXTURE_2D, downsample_texture);
                glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB5_A1 as _, downsample_texture_width, downsample_texture_height, 0, GL_RGBA, GL_FLOAT, std::ptr::null());
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as _);

                glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, downsample_texture, 0);
                assert!(glCheckFramebufferStatus(GL_DRAW_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE);

                FBO_CONTEXT = Some((downsample_fbo, downsample_texture));
            }

            if PROGRAM_CONTEXT.is_none() {
                let positions_flat: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0];

                let texcoords_flat: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];

                let vs = shaders::compile_shader(shaders::vertex::RAW_READ, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::RAW_READ, GL_FRAGMENT_SHADER);
                let program = shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, positions_flat.as_ptr() as _, GL_STATIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as _, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, texcoords_flat.as_ptr() as _, GL_STATIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as _, 0, std::ptr::null());

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[]));
            }

            let fbo_context = FBO_CONTEXT.unwrap();
            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo_context.0);
            glViewport(0, 0, downsample_texture_width, downsample_texture_height);

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            glBindVertexArray(program_context.vao_id);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);

            let framebuffer_cstr = b"framebuffer\0";
            let framebuffer_uniform = glGetUniformLocation(program_context.program_id, framebuffer_cstr.as_ptr() as _);
            glUniform1i(framebuffer_uniform, 0);

            glMemoryBarrier(GL_TEXTURE_FETCH_BARRIER_BIT);
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);

            glBindFramebuffer(GL_DRAW_FRAMEBUFFER, SCENE_FBO);
            glViewport(0, 0, SCENE_TEXTURE_WIDTH, SCENE_TEXTURE_HEIGHT);
        }
    }

    let mut raw_buffer: Vec<u16> = vec![0; VRAM_WIDTH_16B * VRAM_HEIGHT_LINES];

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            let fbo_context = FBO_CONTEXT.unwrap();
            glBindTexture(GL_TEXTURE_2D, fbo_context.1);
            glMemoryBarrier(GL_TEXTURE_UPDATE_BARRIER_BIT);
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_SHORT_1_5_5_5_REV, raw_buffer.as_mut_ptr() as _);
        }
    }

    let mut buffer = Vec::new();
    buffer.reserve(VRAM_WIDTH_16B * VRAM_HEIGHT_LINES);
    raw_buffer.iter().for_each(|c| buffer.push(PackedColor::new(*c)));
    buffer = extract_rectangle(&buffer, VRAM_WIDTH_16B, rect);
    buffer = flip_rows(&buffer, rect.size.width as usize);

    Ok(buffer)
}

pub(crate) fn write_framebuffer(backend_params: &BackendParams, params: WriteFramebufferParams) -> ControllerResult<()> {
    static mut PROGRAM_CONTEXT: Option<ProgramContext> = None;

    debug::trace_call(stdext::function_name!());

    let positions_flat = make_triangle_fan(params.rectangle);
    let mask_bit_force_set_value = bool_to_flag(params.mask_bit_force_set) as i32;
    let mask_bit_check_value = bool_to_flag(params.mask_bit_check) as i32;

    let mut buffer = Vec::new();
    buffer.reserve(params.rectangle.size.width as usize * params.rectangle.size.height as usize);
    params.data.iter().for_each(|pc| buffer.push(PackedColor::new(pc.color)));
    buffer = flip_rows(&buffer, params.rectangle.size.width as usize);

    {
        let (_context_guard, _context) = backend_params.context.guard();

        unsafe {
            if PROGRAM_CONTEXT.is_none() {
                let texcoords_flat: [f32; 8] = [1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0];

                let vs = shaders::compile_shader(shaders::vertex::RAW_WRITE, GL_VERTEX_SHADER);
                let fs = shaders::compile_shader(shaders::fragment::RAW_WRITE, GL_FRAGMENT_SHADER);
                let program = shaders::create_program(&[vs, fs]);

                let mut vao = 0;
                glGenVertexArrays(1, &mut vao);
                glBindVertexArray(vao);
                glEnableVertexAttribArray(0);
                glEnableVertexAttribArray(1);

                let mut vbo_position = 0;
                glGenBuffers(1, &mut vbo_position);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_position);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, std::ptr::null(), GL_DYNAMIC_DRAW);
                glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE as _, 0, std::ptr::null());

                let mut vbo_texcoord = 0;
                glGenBuffers(1, &mut vbo_texcoord);
                glBindBuffer(GL_ARRAY_BUFFER, vbo_texcoord);
                glBufferData(GL_ARRAY_BUFFER, (8 * std::mem::size_of::<f32>()) as _, texcoords_flat.as_ptr() as _, GL_STATIC_DRAW);
                glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE as _, 0, std::ptr::null());

                let mut texture = 0;
                glGenTextures(1, &mut texture);
                glBindTexture(GL_TEXTURE_2D, texture);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as _);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as _);

                PROGRAM_CONTEXT = Some(ProgramContext::new(program, vao, &[vbo_position, vbo_texcoord], &[texture]));
            }

            let program_context = PROGRAM_CONTEXT.as_ref().unwrap();
            glUseProgram(program_context.program_id);
            
            glBindVertexArray(program_context.vao_id);

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, SCENE_TEXTURE);

            let framebuffer_cstr = b"framebuffer\0";
            let framebuffer_uniform = glGetUniformLocation(program_context.program_id, framebuffer_cstr.as_ptr() as _);
            glUniform1i(framebuffer_uniform, 0);
            
            glActiveTexture(GL_TEXTURE1);
            glBindTexture(GL_TEXTURE_2D, program_context.texture_ids[0]);
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as _, params.rectangle.size.width as _, params.rectangle.size.height as _, 0, GL_RGBA, GL_UNSIGNED_SHORT_1_5_5_5_REV, buffer.as_ptr() as _);

            let raw_texture_cstr = b"raw_texture\0";
            let raw_texture_uniform = glGetUniformLocation(program_context.program_id, raw_texture_cstr.as_ptr() as _);
            glUniform1i(raw_texture_uniform, 1);
            
            let mask_bit_force_set_cstr = b"mask_bit_force_set\0";
            let mask_bit_force_set_uniform = glGetUniformLocation(program_context.program_id, mask_bit_force_set_cstr.as_ptr() as _);
            glUniform1i(mask_bit_force_set_uniform, mask_bit_force_set_value);

            let mask_bit_check_cstr = b"mask_bit_check\0";
            let mask_bit_check_uniform = glGetUniformLocation(program_context.program_id, mask_bit_check_cstr.as_ptr() as _);
            glUniform1i(mask_bit_check_uniform, mask_bit_check_value);

            glBindBuffer(GL_ARRAY_BUFFER, program_context.vbo_ids[0]);
            glBufferSubData(GL_ARRAY_BUFFER, 0, (8 * std::mem::size_of::<f32>()) as _, positions_flat.as_ptr() as _);

            glTextureBarrier();
            glDrawArrays(GL_TRIANGLE_FAN, 0, 4);
        }
    }

    Ok(())
}
