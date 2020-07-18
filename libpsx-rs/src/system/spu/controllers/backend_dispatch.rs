#![allow(unused_variables)]

#[cfg(openal)]
mod openal;

use crate::{
    backends::audio::AudioBackend,
    types::stereo::Stereo,
};

pub(crate) fn play_pcm_samples(audio_backend: &AudioBackend, sample_buffer: &[Stereo], voice_id: usize) -> Result<Result<(), ()>, String> {
    match audio_backend {
        AudioBackend::None => Ok(Err(())),
        #[cfg(openal)]
        AudioBackend::Openal(ref backend_params) => Ok(Ok(openal::play_pcm_samples(backend_params, sample_buffer, voice_id)?)),
        _ => unimplemented!(),
    }
}
