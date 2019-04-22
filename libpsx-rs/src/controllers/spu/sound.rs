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
use crate::resources::spu::voice::*;

pub unsafe fn generate_sound(state: &State) {
    for voice_id in 0..24 {
        let play_state = &mut *get_play_state(state, voice_id);

        handle_play_initialization(state, voice_id);
        
        if !play_state.playing {
            continue;
        }

        let adpcm_sample_index = Bitfield::new(12, 20).extract_from(play_state.pitch_counter);

        if adpcm_sample_index == 0 {
            if voice_id == 0 { debug!("decoding new block"); }
            decode_adpcm_block(state, voice_id);
        }

        let adpcm_sample = play_state.adpcm_state.sample_buffer[adpcm_sample_index as usize];
        let mut pcm_frame = handle_volume_transform(state, voice_id, adpcm_sample);
        pcm_frame = handle_main_volume_transform(state, pcm_frame);
        play_state.sample_buffer.push(pcm_frame);
        
        let sample_rate = &mut *get_adpcm_sr(state, voice_id);
        let sample_rate_value = sample_rate.read_u16() as u32;
        if voice_id == 0 {
            debug!("adpcm_sample_index: {}, sample_rate: 0x{:4X}, pitch_counter: 0x{:4X}", adpcm_sample_index, sample_rate_value, play_state.pitch_counter);
        }
        play_state.pitch_counter += sample_rate_value;
        let sample_index_bitfield = Bitfield::new(12, 20);
        let adpcm_sample_index = sample_index_bitfield.extract_from(play_state.pitch_counter) % 28;
        play_state.pitch_counter = sample_index_bitfield.insert_into(play_state.pitch_counter, adpcm_sample_index);

        handle_play_sound_buffer(state, voice_id, false);

        handle_play_termination(state, voice_id);
    }
}

unsafe fn handle_play_initialization(state: &State, voice_id: usize) {
    let resources = &mut *state.resources;
    let key_on = &mut resources.spu.voice_key_on;
    let key_off = &mut resources.spu.voice_key_off;
    let status = &mut resources.spu.voice_channel_status;
    let play_state = &mut *get_play_state(state, voice_id);
    let start_address = &mut *get_adpcm_sa(state, voice_id);
    let voice_bitfield = Bitfield::new(voice_id, 1);

    let _key_on_lock = key_on.mutex.lock();
    let _key_off_lock = key_off.mutex.lock();

    let voice_on = key_on.write_latch[voice_id] && key_on.register.read_bitfield(Bitfield::new(voice_id, 1)) > 0;

    if voice_on {
        play_state.playing = true;
        play_state.current_address = start_address.read_u16() as usize * 8;
        play_state.adpcm_state = AdpcmState::new();
        play_state.pitch_counter = 0;
        play_state.old_sample = 0;
        play_state.older_sample = 0;
        play_state.oldest_sample = 0;
        
        initialize_sound_buffer(state, voice_id);

        status.write_bitfield(voice_bitfield, 0);

        //debug!("Voice {}: key on, current address = 0x{:X}", voice_id, play_state.current_address);
        key_on.write_latch[voice_id] = false;
    }
}

unsafe fn handle_play_termination(state: &State, voice_id: usize) {
    let resources = &mut *state.resources;
    let key_off = &mut resources.spu.voice_key_off;
    let play_state = &mut *get_play_state(state, voice_id);

    let _key_off_lock = key_off.mutex.lock();

    let voice_off = key_off.write_latch[voice_id] && key_off.register.read_bitfield(Bitfield::new(voice_id, 1)) > 0;

    if voice_off {
        handle_play_sound_buffer(state, voice_id, true);

        play_state.playing = false;
        //debug!("Voice {}: key off, current address = 0x{:X}", voice_id, play_state.current_address);
        key_off.write_latch[voice_id] = false;
    }
}

unsafe fn handle_play_sound_buffer(state: &State, voice_id: usize, force: bool) {
    let play_state = &mut *get_play_state(state, voice_id);

    let forced = force && play_state.sample_buffer.len() > 0;

    if (play_state.sample_buffer.len() == BUFFER_SIZE) || forced {
        //let sample_rate = &mut *get_adpcm_sr(state, voice_id);
        //debug!("Playing sound [{}], sample rate should be {:X}h", voice_id, sample_rate.read_u16());

        // TODO: proper frequency, although pcsxr just assumes 44100 all the time...
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

    // On next block decode, process the previous block's parameters.
    if play_state.adpcm_state.params.loop_start {
        repeat_address.write_u16((play_state.current_address / 8) as u16);
    }

    if play_state.adpcm_state.params.loop_end {
        play_state.current_address = (repeat_address.read_u16() as usize * 8) & 0x7FFFF;
        status.write_bitfield(Bitfield::new(voice_id, 1), 1);
    } else {
        play_state.current_address = (play_state.current_address + 16) & 0x7FFFF;
    }

    if !play_state.adpcm_state.params.loop_repeat {
        // Set ADSR to release by writing 0x0000
    }

    let header = [memory.read_u8(play_state.current_address), memory.read_u8(play_state.current_address + 1)];
    play_state.adpcm_state.params = decode_header(header);

    // ADPCM (packed) samples are from indexes 2 -> 15, with each byte containing 2 real samples.
    for i in 0..14 {
        let data = memory.read_u8(play_state.current_address + (2 + i));
        let samples = decode_frame(data, &play_state.adpcm_state.params, &mut play_state.adpcm_state.old_sample, &mut play_state.adpcm_state.older_sample);
        play_state.adpcm_state.sample_buffer[i * 2] = samples[0];
        play_state.adpcm_state.sample_buffer[(i * 2) + 1] = samples[1];
    }
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
