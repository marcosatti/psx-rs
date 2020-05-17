use hashbrown::HashMap;
use lazy_static::lazy_static;
use opengl_sys::*;
use parking_lot::Mutex;
use std::ffi::CString;

pub(crate) mod vertex {
    pub(crate) const SOLID_POLYGON: &'static str = include_str!("./shaders/vertex/solid_polygon.glsl");
    pub(crate) const SHADED_POLYGON: &'static str = include_str!("./shaders/vertex/shaded_polygon.glsl");
    pub(crate) const TEXTURED_POLYGON: &'static str = include_str!("./shaders/vertex/textured_polygon.glsl");
}

pub(crate) mod fragment {
    pub(crate) const SOLID_POLYGON: &'static str = include_str!("./shaders/fragment/solid_polygon.glsl");
    pub(crate) const SHADED_POLYGON: &'static str = include_str!("./shaders/fragment/shaded_polygon.glsl");
    pub(crate) const TEXTURED_POLYGON: &'static str = include_str!("./shaders/fragment/textured_polygon.glsl");
}

lazy_static! {
    static ref SHADERS: Mutex<HashMap<&'static str, GLuint>> = Mutex::new(HashMap::new());
}

pub(crate) fn compile_shader(code: &'static str, type_: GLenum) -> GLuint {
    let mut shaders = SHADERS.lock();

    match shaders.get(code) {
        Some(&id) => id,
        None => {
            let id = unsafe {
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
            };
            shaders.insert(code, id);
            id
        },
    }
}

pub(crate) fn create_program(shaders: &[GLuint]) -> GLuint {
    unsafe {
        let program_id = glCreateProgram();
        for &id in shaders.iter() {
            glAttachShader(program_id, id);
        }
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
