use crate::{
    backends::audio::AudioBackend,
    system::{
        spu::{
            constants::*,
            controllers::{
                adpcm::*,
                adsr::*,
                backend_dispatch,
                interpolation::*,
                voice::*,
                volume::*,
            },
            types::*,
        },
        types::State,
    },
    types::bitfield::Bitfield,
};

pub fn generate_sound(state: &State, spu_state: &mut ControllerState, audio_backend: &AudioBackend) {
    let pmon_value = state.spu.voice_channel_fm.read_u32();
    if pmon_value > 0 {
        unimplemented!("Pitch modulation not implemented: 0x{:X}", pmon_value);
    }

    let noise_value = state.spu.voice_channel_noise.read_u32();
    if noise_value > 0 {
        unimplemented!("Noise generation not implemented: 0x{:X}", noise_value);
    }

    for voice_id in 0..24 {
        handle_key_on(state, spu_state, voice_id);

        let need_data = {
            let play_state = get_play_state(spu_state, voice_id);
            play_state.adpcm_state.sample_buffer.is_none()
        };

        if need_data {
            decode_adpcm_block(state, spu_state, voice_id);
        }

        let adpcm_sample_raw = {
            let play_state = get_play_state(spu_state, voice_id);
            let adpcm_sample_buffer = play_state.adpcm_state.sample_buffer.as_ref().unwrap();
            let adpcm_sample_raw = adpcm_sample_buffer[play_state.pitch_counter_base];
            interpolate_sample(
                adpcm_sample_raw,
                &mut play_state.old_sample,
                &mut play_state.older_sample,
                &mut play_state.oldest_sample,
                play_state.pitch_counter_interp
            )
        };

        handle_pitch_counter(state, spu_state, voice_id);

        handle_adsr_envelope(state, spu_state, voice_id);

        // The incoming ADPCM sample (mono) is volume transformed 3 times, and turned into stereo.
        let adpcm_sample = transform_voice_adsr_volume(spu_state, voice_id, adpcm_sample_raw);
        let mut pcm_frame = transform_voice_volume(state, voice_id, adpcm_sample);
        pcm_frame = transform_main_volume(state, pcm_frame);

        // All processing done, ready to be played.
        {
            let play_state = get_play_state(spu_state, voice_id);
            play_state.sample_buffer.push(pcm_frame);
        }
        handle_play_sound_buffer(state, spu_state, audio_backend, voice_id);

        handle_key_off(state, spu_state, voice_id);
    }
}

fn handle_key_on(state: &State, spu_state: &mut ControllerState, voice_id: usize) {
    let play_state = get_play_state(spu_state, voice_id);
    let start_address = get_saddr(state, voice_id);
    let key_on = &state.spu.voice_key_on;
    let key_off = &state.spu.voice_key_off;
    let status = &state.spu.voice_channel_status;

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let key_on_write_latches = &mut key_on.write_latch.lock();
    let key_off_write_latches = &mut key_off.write_latch.lock();

    let key_on_value = key_on_write_latches[voice_id] && key_on.register.read_bitfield(voice_bitfield) > 0;

    if key_on_value {
        let current_address = start_address.read_u16() as usize * 8;
        play_state.reset(current_address);

        key_off.register.write_bitfield(voice_bitfield, 0);
        key_off_write_latches[voice_id] = false;

        status.write_bitfield(voice_bitfield, 0);

        key_on_write_latches[voice_id] = false;
    }
}

fn handle_key_off(state: &State, spu_state: &mut ControllerState, voice_id: usize) {
    let play_state = get_play_state(spu_state, voice_id);
    let key_off = &state.spu.voice_key_off;
    let key_off_write_latches = &mut key_off.write_latch.lock();

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let key_off_value = key_off_write_latches[voice_id] && key_off.register.read_bitfield(voice_bitfield) > 0;

    if key_off_value {
        play_state.adsr_mode = AdsrMode::Release;
        key_off_write_latches[voice_id] = false;
    }
}

fn handle_play_sound_buffer(state: &State, spu_state: &mut ControllerState, audio_backend: &AudioBackend, voice_id: usize) {
    let play_state = get_play_state(spu_state, voice_id);
    let control = &state.spu.control;

    if play_state.sample_buffer.len() == BUFFER_SIZE {
        let unmuted = control.read_bitfield(CONTROL_UNMUTE) != 0;

        if unmuted {
            let _ = backend_dispatch::play_pcm_samples(audio_backend, &play_state.sample_buffer, voice_id);
        }

        play_state.sample_buffer.clear();
    }
}

fn decode_adpcm_block(state: &State, spu_state: &mut ControllerState, voice_id: usize) {
    let current_address = {
        let play_state = get_play_state(spu_state, voice_id);
        play_state.current_address
    };

    let header = {
        let memory = &spu_state.memory;
        [memory[current_address], memory[current_address + 1]]
    };
    

    // ADPCM header.
    {
        let play_state = get_play_state(spu_state, voice_id);
        play_state.adpcm_state.params = decode_header(header);
    }

    // ADPCM (packed) samples are from indexes 2 -> 15, with each byte containing 2 real samples.
    let mut sample_buffer = [0; 28];
    for i in 0..14 {
        let data = {
            let memory = &spu_state.memory;
            memory[current_address + (2 + i)]
        };
        let samples = {
            let play_state = get_play_state(spu_state, voice_id);
            decode_frame(data, &play_state.adpcm_state.params, &mut play_state.adpcm_state.old_sample, &mut play_state.adpcm_state.older_sample)
        };
        sample_buffer[i * 2] = samples[0];
        sample_buffer[(i * 2) + 1] = samples[1];
    }


    let play_state = get_play_state(spu_state, voice_id);
    play_state.adpcm_state.sample_buffer = Some(sample_buffer);

    let mut next_address = (current_address + 16) & 0x7FFFF;

    let repeat_address = get_raddr(state, voice_id);
    let status = &state.spu.voice_channel_status;

    // Process header flags.
    if play_state.adpcm_state.params.loop_start {
        repeat_address.write_u16((current_address / 8) as u16);
    }

    if play_state.adpcm_state.params.loop_end {
        next_address = (repeat_address.read_u16() as usize * 8) & 0x7FFFF;
        status.write_bitfield(Bitfield::new(voice_id, 1), 1);

        if !play_state.adpcm_state.params.loop_repeat {
            play_state.adsr_mode = AdsrMode::Release;
        }
    }

    play_state.current_address = next_address;
}

fn handle_pitch_counter(state: &State, spu_state: &mut ControllerState, voice_id: usize) {
    let play_state = get_play_state(spu_state, voice_id);
    let sample_rate = get_srate(state, voice_id);

    let sample_rate_value = sample_rate.read_u16() as u32;
    let interp_value = Bitfield::new(0, 12).extract_from(sample_rate_value) as usize;
    let base_value = Bitfield::new(12, 4).extract_from(sample_rate_value) as usize;

    play_state.pitch_counter_base += base_value;
    play_state.pitch_counter_interp += interp_value;

    if play_state.pitch_counter_interp >= 0x1000 {
        play_state.pitch_counter_interp -= 0x1000;
        play_state.pitch_counter_base += 1;
    }

    if play_state.pitch_counter_base >= 28 {
        play_state.pitch_counter_base -= 28;
        // We need a new block to decode and get samples from.
        play_state.adpcm_state.sample_buffer = None;
    }
}
