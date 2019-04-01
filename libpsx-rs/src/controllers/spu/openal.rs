use openal_sys::*;
use crate::backends::audio::openal::*;
use crate::backends::audio::openal::rendering::*;

pub fn play_pcm_samples(backend_params: &BackendParams, samples: &[i16], _frequency: usize, voice_id: usize) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {        
        let buffer_index = if !RENDERING_ODD_BUFFER[voice_id] {
            voice_id * 2
        } else {
            voice_id * 2 + 1
        };

        alBufferData(BUFFERS[buffer_index], AL_FORMAT_MONO16 as ALenum, samples.as_ptr() as *const std::ffi::c_void, samples.len() as ALsizei, 44100);
        alSourcei(SOURCES[voice_id], AL_BUFFER as ALenum, BUFFERS[buffer_index] as ALint);
        alSourcePlay(SOURCES[voice_id]);

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error in OpenAL audio backend: playing source");
        }

        RENDERING_ODD_BUFFER[voice_id] = !RENDERING_ODD_BUFFER[voice_id];
    }
}
