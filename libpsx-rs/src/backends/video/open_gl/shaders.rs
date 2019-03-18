use std::collections::HashMap;
use std::ffi::CString;
use opengl_sys::*;

pub mod vertex {
    pub const SOLID_POLYGON: &'static str = include_str!("./shaders/vertex/solid_polygon.glsl");
    pub const SHADED_POLYGON: &'static str = include_str!("./shaders/vertex/shaded_polygon.glsl");
    pub const TEXTURED_POLYGON: &'static str = include_str!("./shaders/vertex/textured_polygon.glsl");
}

pub mod fragment {
    pub const SOLID_POLYGON: &'static str = include_str!("./shaders/fragment/solid_polygon.glsl");
    pub const SHADED_POLYGON: &'static str = include_str!("./shaders/fragment/shaded_polygon.glsl");
    pub const TEXTURED_POLYGON: &'static str = include_str!("./shaders/fragment/textured_polygon.glsl");
}

static mut PROGRAMS: Option<HashMap<(*mut std::ffi::c_void, &'static str, &'static str), GLuint>> = None; 

pub fn get_program(context: &sdl2::video::GLContext, vertex: &'static str, fragment: &'static str) -> GLuint {
    unsafe {
        if PROGRAMS.is_none() {
            PROGRAMS = Some(HashMap::new());
        }

        match PROGRAMS.as_ref().unwrap().get(&(context.raw(), vertex, fragment)) {
            Some(&p) => p,
            None => {
                let vertex_shader_id = compile_shader(vertex, GL_VERTEX_SHADER);
                let fragment_shader_id = compile_shader(fragment, GL_FRAGMENT_SHADER);
                let program_id = create_program(&[vertex_shader_id, fragment_shader_id]);
                glDetachShader(program_id, vertex_shader_id);
                glDetachShader(program_id, fragment_shader_id);
                glDeleteShader(vertex_shader_id);
                glDeleteShader(fragment_shader_id);
                PROGRAMS.as_mut().unwrap().insert((context.raw(), vertex, fragment), program_id);
                program_id
            }
        }
    }
}

fn compile_shader(code: &str, type_: GLenum) -> GLuint {
    unsafe {
        let shader_id = glCreateShader(type_);
        let shader_code = CString::new(code).unwrap();
        let shader_codes = [shader_code.as_c_str().as_ptr()];
        glShaderSource(shader_id, 1, shader_codes.as_ptr(), std::ptr::null());
        glCompileShader(shader_id);
        
        let mut status = 0;
        glGetShaderiv(shader_id, GL_COMPILE_STATUS, &mut status);
        if status == (GL_FALSE as GLint) {
            let mut log_length = 0;
            glGetShaderiv(shader_id, GL_INFO_LOG_LENGTH, &mut log_length);
            let mut buffer = vec![0_u8; log_length as usize];
            glGetShaderInfoLog(shader_id, log_length, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            panic!("Shader compilation error: {}, code: {}", String::from_utf8(buffer).unwrap(), code);
        }

        shader_id
    }
}

fn create_program(shaders: &[GLuint]) -> GLuint {
    unsafe {
        let program_id = glCreateProgram();
        for &id in shaders.iter() { glAttachShader(program_id, id); }
        glLinkProgram(program_id);

        let mut status = 0;
        glGetProgramiv(program_id, GL_LINK_STATUS, &mut status);
        if status == (GL_FALSE as GLint) {
            let mut log_length = 0;
            glGetProgramiv(program_id, GL_INFO_LOG_LENGTH, &mut log_length);
            let mut buffer = vec![0_u8; log_length as usize];
            glGetProgramInfoLog(program_id, log_length, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            panic!("Program link error: {}", String::from_utf8(buffer).unwrap());
        }

        program_id
    }
}
