use openal_sys::*;
use crate::backends::audio::open_al::*;

// TODO: very quick hack to get things working

static mut BUFFER: [ALuint; 24] = [0; 24];
static mut SOURCE: [ALuint; 24] = [0; 24];

pub fn play_pcm_samples(backend_params: &BackendParams, samples: &[i16], _frequency: usize, voice_id: usize) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {        
        if SOURCE[voice_id] != 0 {
            alSourceStop(SOURCE[voice_id]);
            alSourcei(SOURCE[voice_id], AL_BUFFER as ALenum, 0);
            alDeleteSources(1, &mut SOURCE[voice_id]);
            SOURCE[voice_id] = 0;
        }

        if BUFFER[voice_id] != 0 {
            alDeleteBuffers(1, &mut SOURCE[voice_id]);
            BUFFER[voice_id] = 0;
        }

        alGenBuffers(1, &mut BUFFER[voice_id]);
        alGenSources(1, &mut SOURCE[voice_id]);

        alBufferData(BUFFER[voice_id], AL_FORMAT_MONO16 as ALenum, samples.as_ptr() as *const std::ffi::c_void, samples.len() as ALsizei, 44100);
        alSourcei(SOURCE[voice_id], AL_BUFFER as ALenum, BUFFER[voice_id] as ALint);
        alSourcePlay(SOURCE[voice_id]);
    }
}
