use log::{debug, warn};
use crate::constants::spu::dac::*;
use crate::State;
use crate::backends::audio::AudioBackend;
use crate::types::bitfield::Bitfield;
use crate::types::register::b16_register::*;
use crate::types::stereo::*;
use crate::controllers::spu::voice::*;
use crate::controllers::spu::adpcm::*;
use crate::controllers::spu::openal;
use crate::controllers::spu::volume::*;
use crate::controllers::spu::adsr::*;
use crate::resources::spu::voice::*;

pub unsafe fn generate_sound(state: &State) {
    for voice_id in 0..24 {
        let play_state = &mut *get_play_state(state, voice_id);

        handle_key_on(state, voice_id);

        if play_state.adpcm_state.sample_buffer.is_none() {
            decode_adpcm_block(state, voice_id);
        }

        let adpcm_sample_buffer = play_state.adpcm_state.sample_buffer.as_ref().unwrap();
        let adpcm_sample_raw = adpcm_sample_buffer[play_state.pitch_counter_base];

        handle_pitch_counter_update(state, voice_id);

        handle_adsr_envelope(state, voice_id);
        let adpcm_sample = ((adpcm_sample_raw as i32 * play_state.adsr_current_volume as i32) / std::i16::MAX as i32) as i16;

        // The incoming ADPCM sample (mono) is transformed into stereo through volume transformations. 
        // This wasn't very clear in the docs...
        let mut pcm_frame = handle_volume_transform(state, voice_id, adpcm_sample);
        pcm_frame = handle_main_volume_transform(state, pcm_frame);

        // All processing done, ready to be played.
        play_state.sample_buffer.push(pcm_frame);
        handle_play_sound_buffer(state, voice_id);

        handle_key_off(state, voice_id);
    }
}

unsafe fn handle_key_on(state: &State, voice_id: usize) {
    let resources = &mut *state.resources;
    let key_on = &mut resources.spu.voice_key_on;
    let key_off = &mut resources.spu.voice_key_off;
    let status = &mut resources.spu.voice_channel_status;
    let play_state = &mut *get_play_state(state, voice_id);
    let start_address = &mut *get_adpcm_sa(state, voice_id);

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let _key_on_lock = key_on.mutex.lock();
    let _key_off_lock = key_off.mutex.lock();

    let key_on_value = key_on.write_latch[voice_id] && key_on.register.read_bitfield(voice_bitfield) > 0;

    if key_on_value {
        *play_state = PlayState::new();
        play_state.current_address = start_address.read_u16() as usize * 8;
        play_state.adpcm_state = AdpcmState::new();
        play_state.pitch_counter_base = 0;
        play_state.pitch_counter_interp = 0;
        play_state.old_sample = 0;
        play_state.older_sample = 0;
        play_state.oldest_sample = 0;
        play_state.adsr_mode = AdsrMode::Attack;
        play_state.adsr_current_volume = 0;
        initialize_sound_buffer(state, voice_id);

        key_off.register.write_bitfield(voice_bitfield, 0);
        key_off.write_latch[voice_id] = false;

        status.write_bitfield(voice_bitfield, 0);

        key_on.write_latch[voice_id] = false;
    }
}

unsafe fn handle_key_off(state: &State, voice_id: usize) {
    let resources = &mut *state.resources;
    let key_off = &mut resources.spu.voice_key_off;
    let play_state = &mut *get_play_state(state, voice_id);

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let _key_off_lock = key_off.mutex.lock();

    let key_off_value = key_off.write_latch[voice_id] && key_off.register.read_bitfield(voice_bitfield) > 0;

    if key_off_value {
        play_state.adsr_mode = AdsrMode::Release;
        key_off.write_latch[voice_id] = false;
    }
}

unsafe fn handle_play_sound_buffer(state: &State, voice_id: usize) {
    let play_state = &mut *get_play_state(state, voice_id);

    if play_state.sample_buffer.len() == BUFFER_SIZE {
        match state.audio_backend {
            AudioBackend::Openal(ref backend_params) => {
                openal::play_pcm_samples(backend_params, &play_state.sample_buffer, voice_id);
            },
        }

        initialize_sound_buffer(state, voice_id);
    }
}

unsafe fn initialize_sound_buffer(state: &State, voice_id: usize) {
    let play_state = &mut *get_play_state(state, voice_id);
    play_state.sample_buffer = Vec::with_capacity(play_state.sample_buffer.capacity());
}

unsafe fn decode_adpcm_block(state: &State, voice_id: usize) {
    let resources = &mut *state.resources;
    let memory = &resources.spu.memory;
    let play_state = &mut *get_play_state(state, voice_id);
    let repeat_address = &mut *get_adpcm_ra(state, voice_id);
    let status = &mut resources.spu.voice_channel_status;

    // ADPCM header.
    let header = [memory.read_u8(play_state.current_address), memory.read_u8(play_state.current_address + 1)];
    play_state.adpcm_state.params = decode_header(header);

    // ADPCM (packed) samples are from indexes 2 -> 15, with each byte containing 2 real samples.
    let mut sample_buffer = [0; 28];
    for i in 0..14 {
        let data = memory.read_u8(play_state.current_address + (2 + i));
        let samples = decode_frame(data, &play_state.adpcm_state.params, &mut play_state.adpcm_state.old_sample, &mut play_state.adpcm_state.older_sample);
        sample_buffer[i * 2] = samples[0];
        sample_buffer[(i * 2) + 1] = samples[1];
    }
    play_state.adpcm_state.sample_buffer = Some(sample_buffer);

    let mut next_address = (play_state.current_address + 16) & 0x7FFFF;

    // Process header flags.
    if play_state.adpcm_state.params.loop_start {
        repeat_address.write_u16((play_state.current_address / 8) as u16);
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

unsafe fn handle_volume_transform(state: &State, voice_id: usize, adpcm_sample: i16) -> Stereo {
    let vol_left = &mut *get_voll(state, voice_id);
    let vol_right = &mut *get_volr(state, voice_id);

    let process_sample = |vol: &mut B16Register| -> i16 {
        let vol_value = vol.read_u16();
        let volume_mode = Bitfield::new(15, 1).extract_from(vol_value);
        if volume_mode != 0 {
            let (sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode) = extract_sweep_params(vol_value);
            transform_sample_sweep(adpcm_sample, sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode)
        } else {
            let volume15 = Bitfield::new(0, 15).extract_from(vol_value);
            transform_sample_fixed(adpcm_sample, volume15)
        }
    };

    let left_sample = process_sample(vol_left);
    let right_sample = process_sample(vol_right);

    Stereo::new(left_sample, right_sample)
}

unsafe fn handle_main_volume_transform(state: &State, pcm_frame: Stereo) -> Stereo {
    let resources = &mut *state.resources;
    let mvol_left = &mut resources.spu.main_volume_left;
    let mvol_right = &mut resources.spu.main_volume_right;

    let process_sample = |sample, mvol: &mut B16Register| -> i16 {
        let mvol_value = mvol.read_u16();
        let volume_mode = Bitfield::new(15, 1).extract_from(mvol_value);
        if volume_mode != 0 {
            let (sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode) = extract_sweep_params(mvol_value);
            transform_sample_sweep(sample, sweep_step, sweep_shift, sweep_phase, sweep_direction, sweep_mode)
        } else {
            let volume15 = Bitfield::new(0, 15).extract_from(mvol_value);
            transform_sample_fixed(sample, volume15)
        }
    };

    let left_sample = process_sample(pcm_frame.left, mvol_left);
    let right_sample = process_sample(pcm_frame.right, mvol_right);

    Stereo::new(left_sample, right_sample)
}

unsafe fn handle_pitch_counter_update(state: &State, voice_id: usize) {
    let play_state = &mut *get_play_state(state, voice_id);
    let sample_rate = &mut *get_adpcm_sr(state, voice_id);

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
