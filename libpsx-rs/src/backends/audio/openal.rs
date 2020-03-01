pub mod rendering;

use openal_sys::*;
use crate::backends::context::*;

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, *mut ALCcontext>,
}

pub fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        alGenSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());
        alGenBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error initializing OpenAL audio backend");
        }
    }
}

pub fn teardown(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        alDeleteBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());
        alDeleteSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error tearing down OpenAL audio backend");
        }
    }
}
