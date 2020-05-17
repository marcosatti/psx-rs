pub(crate) mod rendering;

use crate::{
    backends::{
        audio::openal::rendering::*,
        context::*,
    },
    system::spu::constants::VOICES_COUNT,
};
use openal_sys::*;

static mut INITIALIZED: bool = false;

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, ()>,
}

pub(crate) fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        alGenSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());
        alGenBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error initializing OpenAL audio backend");
        }

        INITIALIZED = true;
    }
}

pub(crate) fn teardown(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            for i in 0..VOICES_COUNT {
                alSourceStop(SOURCES[i]);
                alSourcei(SOURCES[i], AL_BUFFER as ALenum, 0);
            }

            alDeleteBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());
            alDeleteSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());

            INITIALIZED = false;
        }
    }
}
