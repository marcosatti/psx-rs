pub mod adpcm;
pub mod adsr;
pub mod interpolation;
pub mod voice;
pub mod volume;
pub mod pitch;

use crate::{
    backends::audio::AudioBackend,
    system::{
        spu::{
            constants::*,
            controllers::{
                backend_dispatch,
                dac::adpcm::*,
                dac::adsr::*,
                dac::interpolation::*,
                dac::voice::*,
                dac::volume::*,
                dac::pitch::*,
            },
            types::*,
        },
        types::State,
    },
};

pub fn handle_dac(state: &State, controller_state: &mut ControllerState, audio_backend: &AudioBackend, voice_id: usize) {
    handle_adpcm_block(state, controller_state, voice_id);

    let adpcm_sample_raw = handle_interpolation(controller_state, voice_id);

    handle_pitch_counter(state, controller_state, voice_id);

    handle_adsr_envelope(state, controller_state, voice_id);

    let pcm_frame = apply_sample_volume(state, controller_state, voice_id, adpcm_sample_raw);

    get_voice_state(controller_state, voice_id).sample_buffer.push(pcm_frame);

    handle_play_sound_buffer(controller_state, audio_backend, voice_id);
}

fn handle_play_sound_buffer(controller_state: &mut ControllerState, audio_backend: &AudioBackend, voice_id: usize) {
    let muted = controller_state.muted;
    let play_state = get_voice_state(controller_state, voice_id);

    if play_state.sample_buffer.len() == BUFFER_SIZE {
        if !muted {
            let _ = backend_dispatch::play_pcm_samples(audio_backend, &play_state.sample_buffer, voice_id);
        }

        play_state.sample_buffer.clear();
    }
}
