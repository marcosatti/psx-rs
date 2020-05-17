use crate::{
    system::{
        dmac::{
            constants::*,
            controllers::fifo::*,
            types::*,
        },
        types::State,
    },
    types::bitfield::Bitfield,
};

pub(crate) fn handle_transfer(state: &State, linked_list_state: &mut LinkedListState, channel_id: usize) -> Result<(bool, bool), ()> {
    let remaining = linked_list_state.target_count - linked_list_state.current_count;

    if remaining == 0 {
        if linked_list_state.next_header_address == 0xFF_FFFF {
            return Ok((true, true));
        }

        let header_value = state.memory.main_memory.read_u32(linked_list_state.next_header_address);
        let (address, count) = process_header(header_value);

        if address == 0 {
            log::warn!("Linked list transfer: null pointer encountered on channel {}, ending transfer prematurely", channel_id);
            return Ok((true, true));
        }

        linked_list_state.current_header_address = linked_list_state.next_header_address;
        linked_list_state.next_header_address = address;
        linked_list_state.current_count = 0;
        linked_list_state.target_count = count;

        Ok((false, true))
    } else {
        let address_offset = (linked_list_state.current_count as u32) * DATA_SIZE;
        let address = (linked_list_state.current_header_address + DATA_SIZE) + address_offset;

        let value = state.memory.main_memory.read_u32(address as u32);
        push_channel_data(state, channel_id, value)?;

        linked_list_state.current_count += 1;

        Ok((false, false))
    }
}

fn process_header(header_value: u32) -> (u32, usize) {
    const ADDRESS: Bitfield = Bitfield::new(0, 24);
    const COUNT: Bitfield = Bitfield::new(24, 8);

    (ADDRESS.extract_from(header_value), COUNT.extract_from(header_value) as usize)
}
