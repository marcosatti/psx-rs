use crate::{
    backends::audio::openal::{
        rendering::*,
        *,
    },
    system::types::ControllerResult,
    types::stereo::*,
};
use openal_sys::*;

pub(crate) fn play_pcm_samples(backend_params: &BackendParams, samples: &[Stereo], voice_id: usize) -> ControllerResult {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        let buffer_index = if !RENDERING_ODD_BUFFER[voice_id] {
            voice_id * 2
        } else {
            voice_id * 2 + 1
        };

        let samples_size = (samples.len() * std::mem::size_of::<Stereo>()) as ALsizei;
        alBufferData(BUFFERS[buffer_index], AL_FORMAT_STEREO16 as ALenum, samples.as_ptr() as *const std::ffi::c_void, samples_size, 44100);

        alSourceStop(SOURCES[voice_id]);
        alSourcei(SOURCES[voice_id], AL_BUFFER as ALenum, BUFFERS[buffer_index] as ALint);
        alSourcePlay(SOURCES[voice_id]);

        if alGetError() != AL_NO_ERROR as ALenum {
            return Err("Error in OpenAL audio backend: playing source".into());
        }

        RENDERING_ODD_BUFFER[voice_id] = !RENDERING_ODD_BUFFER[voice_id];
    }

    Ok(())
}
