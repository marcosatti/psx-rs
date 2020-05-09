use crate::{
    system::spu::types::*,
    system::spu::constants::*,
    system::spu::controllers::dac::voice::*,
    types::bitfield::Bitfield,
    system::types::State,
};
use num_traits::clamp;

pub fn handle_adpcm_block(state: &State, controller_state: &mut ControllerState, voice_id: usize) {
    let decoding_address = {
        let play_state = get_voice_state(controller_state, voice_id);
        let mut current_address = play_state.current_address;
        let decoded_address = play_state.adpcm_state.decoded_address;
        
        // TODO: must be aligned to 8-byte boundaries? Which is a bit weird, condsidering a block is 16-bytes...
        // Start/repeat registers are anyway... am I missing something here?
        assert_eq!(current_address % 8, 0);
        assert_eq!(decoded_address % 8, 0);

        if current_address == decoded_address {
            return;
        }

        if play_state.adpcm_state.copy_repeat_address {
            current_address = get_raddr(state, voice_id).read_u16() as usize * 8;
            play_state.current_address = current_address;
            play_state.adpcm_state.copy_repeat_address = false;
        }

        current_address
    };

    let block = read_block(&controller_state.memory, decoding_address);
    let play_state = get_voice_state(controller_state, voice_id);
    let params = decode_header(block.header);
    let sample_buffer = decode_all_frames(block.samples, params, &mut play_state.adpcm_state.old_sample, &mut play_state.adpcm_state.older_sample);
    play_state.adpcm_state.decoded_address = decoding_address;
    play_state.adpcm_state.sample_buffer = sample_buffer;

    if params.loop_start {
        get_raddr(state, voice_id).write_u16((decoding_address / 8) as u16);
    }

    if params.loop_end {
        play_state.adpcm_state.copy_repeat_address = true;

        state.spu.voice_channel_status.write_bitfield(Bitfield::new(voice_id, 1), 1);

        if !params.loop_repeat {
            // Set ADSR to release and mute immediately.
            play_state.adsr_state.phase = AdsrPhase::Release;
            play_state.adsr_state.current_volume = 0;
            play_state.adsr_state.next_volume = 0;
            play_state.adsr_state.wait_cycles = 0;
        }
    }
}

fn read_block(memory: &Vec<u8>, address: usize) -> AdpcmBlockRaw {
    let mut header = [0; 2];
    let mut samples = [0; 14];

    // 2 header bytes.
    for i in 0..2 {
        header[i] = memory[address + i];
    }

    // 14 packed bytes (2x14 samples).
    for i in 0..14 {
        samples[i] = memory[address + 2 + i];
    }

    AdpcmBlockRaw {
        header: header,
        samples: samples,
    }
}

fn decode_header(header: [u8; 2]) -> AdpcmParams {
    AdpcmParams {
        shift: ADPCM_SHIFT.extract_from(header[0]) as usize,
        filter: ADPCM_FILTER.extract_from(header[0]) as usize,
        loop_end: ADPCM_LOOP_END.extract_from(header[1]) > 0,
        loop_repeat: ADPCM_LOOP_REPEAT.extract_from(header[1]) > 0,
        loop_start: ADPCM_LOOP_START.extract_from(header[1]) > 0,
    }
}

fn decode_all_frames(raw_block: [u8; 14], params: AdpcmParams, old_sample: &mut i16, older_sample: &mut i16) -> [i16; 28] {
    let mut sample_buffer = [0; 28];

    for i in 0..14 {
        let samples = decode_frame(raw_block[i], params, old_sample, older_sample);
        sample_buffer[i * 2] = samples[0];
        sample_buffer[(i * 2) + 1] = samples[1];
    }

    sample_buffer
}

fn decode_frame(data: u8, params: AdpcmParams, old_sample: &mut i16, older_sample: &mut i16) -> [i16; 2] {
    const POS_FILTER_CONSTANTS: [i32; 5] = [0, 60, 115, 98, 122];
    const NEG_FILTER_CONSTANTS: [i32; 5] = [0, 0, -52, -55, -60];

    let mut samples = [
        Bitfield::new(0, 4).extract_from(data) as i16, 
        Bitfield::new(4, 4).extract_from(data) as i16
    ];

    let pos_filter_value = POS_FILTER_CONSTANTS[params.filter];
    let neg_filter_value = NEG_FILTER_CONSTANTS[params.filter];

    for i in 0..2 {
        let mut shifted_sample = ((samples[i] << 12) as i32) >> params.shift;
        // TODO: change formula to the no cash psx docs way - the pcsxr way is wrong? Only this way for debugging.
        // Right shift != division (pow 2) when negatives... https://en.wikipedia.org/wiki/Arithmetic_shift.
        shifted_sample += ((*old_sample as i32 * pos_filter_value) >> 6) + ((*older_sample as i32 * neg_filter_value) >> 6);

        samples[i] = clamp(shifted_sample, std::i16::MIN as i32, std::i16::MAX as i32) as i16;

        *older_sample = *old_sample;
        *old_sample = samples[i];
    }

    samples
}
