use log::debug;
use opengl_sys::*;
use std::ffi::CStr;

pub extern "C" fn debug_callback(
    _source: GLenum, type_: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar,
    _user_param: *const std::ffi::c_void,
)
{
    unsafe {
        if type_ == GL_DEBUG_TYPE_ERROR_ARB {
            let message = CStr::from_ptr(message);
            debug!("OpenGL error: type: {}, severity = {}, message = {}", type_, severity, message.to_str().unwrap());
        }
    }
}
