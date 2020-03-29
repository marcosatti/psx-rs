use crate::system::dmac::controllers::debug;
use crate::system::dmac::types::LinkedListState;
use crate::types::bitfield::Bitfield;
use crate::types::memory::b8_memory::B8Memory;

pub fn process_header(state: &mut LinkedListState, main_memory: &B8Memory) -> Result<(), ()> {
    let header_value = main_memory.read_u32(state.next_header_address);
    let next_header_address = Bitfield::new(0, 24).extract_from(header_value);

    if next_header_address == 0x0 {
        debug::trace_linked_list_null_header(state.next_header_address);
        return Err(());
    }

    let target_count = Bitfield::new(24, 8).extract_from(header_value) as usize;

    state.current_header_address = state.next_header_address;
    state.next_header_address = next_header_address;
    state.target_count = target_count;
    state.current_count = 0;

    Ok(())
}
