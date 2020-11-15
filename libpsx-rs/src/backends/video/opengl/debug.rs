use opengl_sys::*;
use std::ffi::CStr;

pub(crate) extern "system" fn debug_callback(
    _source: GLenum, type_: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _user_param: *mut std::ffi::c_void,
) {
    unsafe {
        if type_ == GL_DEBUG_TYPE_ERROR {
            let message = CStr::from_ptr(message);
            log::debug!("OpenGL error: type: {}, severity = {}, message:", type_, severity);
            log::debug!("    {}", message.to_str().unwrap());
        }
    }
}
