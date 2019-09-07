use crate::constants::spu::dac::*;
use crate::resources::Resources;
use crate::backends::audio::AudioBackend;
use crate::types::bitfield::Bitfield;
use crate::controllers::spu::voice::*;
use crate::controllers::spu::adpcm::*;
use crate::controllers::spu::openal;
use crate::controllers::spu::volume::*;
use crate::controllers::spu::adsr::*;
use crate::controllers::spu::interpolation::*;
use crate::resources::spu::*;
use crate::resources::spu::voice::*;

pub fn generate_sound<'a>(resources: &mut Resources, audio_backend: &AudioBackend<'a>) {
    let pmon_value = resources.spu.voice_channel_fm.read_u32();
    if pmon_value > 0 {
        unimplemented!("Pitch modulation not implemented: 0x{:X}", pmon_value);
    }

    let noise_value = resources.spu.voice_channel_noise.read_u32();
    if noise_value > 0 {
        unimplemented!("Noise generation not implemented: 0x{:X}", noise_value);
    }

    for voice_id in 0..24 {
        let play_state = unsafe { &mut *get_play_state(resources, voice_id) };

        handle_key_on(resources, voice_id);

        if play_state.adpcm_state.sample_buffer.is_none() {
            decode_adpcm_block(resources, voice_id);
        }

        let adpcm_sample_buffer = play_state.adpcm_state.sample_buffer.as_ref().unwrap();
        let mut adpcm_sample_raw = adpcm_sample_buffer[play_state.pitch_counter_base];
        adpcm_sample_raw = interpolate_sample(adpcm_sample_raw, &mut play_state.old_sample, &mut play_state.older_sample, &mut play_state.oldest_sample, play_state.pitch_counter_interp);

        handle_pitch_counter(resources, voice_id);

        handle_adsr_envelope(resources, voice_id);

        // The incoming ADPCM sample (mono) is volume transformed 3 times, and turned into stereo. 
        let adpcm_sample = transform_voice_adsr_volume(resources, voice_id, adpcm_sample_raw);
        let mut pcm_frame = transform_voice_volume(resources, voice_id, adpcm_sample);
        pcm_frame = transform_main_volume(resources, pcm_frame);

        // All processing done, ready to be played.
        play_state.sample_buffer.push(pcm_frame);
        handle_play_sound_buffer(resources, audio_backend, voice_id);

        handle_key_off(resources, voice_id);
    }
}

fn handle_key_on(resources: &mut Resources, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    let start_address = unsafe { &mut *get_adpcm_sa(resources, voice_id) };
    let key_on = &mut resources.spu.voice_key_on;
    let key_off = &mut resources.spu.voice_key_off;
    let status = &mut resources.spu.voice_channel_status;

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let _key_on_lock = key_on.mutex.lock();
    let _key_off_lock = key_off.mutex.lock();

    let key_on_value = key_on.write_latch[voice_id] && key_on.register.read_bitfield(voice_bitfield) > 0;

    if key_on_value {
        let current_address = start_address.read_u16() as usize * 8;
        play_state.reset(current_address);

        key_off.register.write_bitfield(voice_bitfield, 0);
        key_off.write_latch[voice_id] = false;

        status.write_bitfield(voice_bitfield, 0);

        key_on.write_latch[voice_id] = false;
    }
}

fn handle_key_off(resources: &mut Resources, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    let key_off = &mut resources.spu.voice_key_off;

    let voice_bitfield = Bitfield::new(voice_id, 1);

    let _key_off_lock = key_off.mutex.lock();

    let key_off_value = key_off.write_latch[voice_id] && key_off.register.read_bitfield(voice_bitfield) > 0;

    if key_off_value {
        play_state.adsr_mode = AdsrMode::Release;
        key_off.write_latch[voice_id] = false;
    }
}

fn handle_play_sound_buffer<'a>(resources: &mut Resources, audio_backend: &AudioBackend<'a>, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    let control = &resources.spu.control;

    if play_state.sample_buffer.len() == BUFFER_SIZE {
        let muted = control.read_bitfield(CONTROL_MUTE) != 0;

        if !muted {
            match audio_backend {
                AudioBackend::Openal(ref backend_params) => {
                    openal::play_pcm_samples(backend_params, &play_state.sample_buffer, voice_id);
                },
            }
        }

        play_state.sample_buffer.clear();
    }
}

fn decode_adpcm_block(resources: &mut Resources, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    let repeat_address = unsafe { &mut *get_adpcm_ra(resources, voice_id) };
    let status = &mut resources.spu.voice_channel_status;
    let memory = &resources.spu.memory;

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

fn handle_pitch_counter(resources: &mut Resources, voice_id: usize) {
    let play_state = unsafe { &mut *get_play_state(resources, voice_id) };
    let sample_rate = unsafe { &mut *get_adpcm_sr(resources, voice_id) };

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
