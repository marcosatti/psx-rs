use crate::types::memory::b8_memory::B8Memory;
use crate::types::bitfield::Bitfield;
use crate::resources::dmac::channel::LinkedListState;

pub fn process_header(state: &mut LinkedListState, main_memory: &B8Memory) {
    let header_value = main_memory.read_u32(state.next_address);
    let next_address = Bitfield::new(0, 24).extract_from(header_value);
    let target_count = Bitfield::new(24, 8).extract_from(header_value) as usize;

    state.current_address = state.next_address;
    state.next_address = next_address;
    state.target_count = target_count;
    state.current_count = 0;
}
