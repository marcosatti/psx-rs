use crate::{
    system::{
        spu::{
            controllers::dac::voice::*,
            types::*,
        },
        types::State,
    },
    types::bitfield::Bitfield,
};

pub fn handle_pitch_counter(state: &State, controller_state: &mut ControllerState, voice_id: usize) {
    const PITCH_COUNTER_INTERP: Bitfield = Bitfield::new(0, 12);
    const PITCH_COUNTER_SAMPLE: Bitfield = Bitfield::new(12, 4);

    let voice_state = get_voice_state(controller_state, voice_id);
    let sample_rate = get_srate(state, voice_id);

    let sample_rate_value = sample_rate.read_u16() as usize;
    let partial_value = PITCH_COUNTER_INTERP.extract_from(sample_rate_value);
    let index_value = PITCH_COUNTER_SAMPLE.extract_from(sample_rate_value);

    voice_state.sample_counter_partial += partial_value;
    let index_delta = voice_state.sample_counter_partial / 0x1000;
    voice_state.sample_counter_partial %= 0x1000;

    voice_state.sample_counter_index += index_value + index_delta;
    let sample_delta = voice_state.sample_counter_index / 28;
    voice_state.sample_counter_index %= 28;

    let current_address_delta = sample_delta * 16;
    voice_state.current_address += current_address_delta;
    voice_state.current_address &= 0x7FFFF;
}
