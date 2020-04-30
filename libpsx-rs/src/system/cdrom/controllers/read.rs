use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::{
                backend_dispatch,
                interrupt::*,
                state::*,
            },
            types::ControllerState,
        },
        types::State,
    },
};

pub fn handle_read(state: &State, cdrom_state: &mut ControllerState, cdrom_backend: &CdromBackend) -> bool {
    // Buffer some data first (INT1 means ready to send data?).
    // Do we always want to send data if we have read a sector regardless of the current reading status? Seems like the
    // BIOS expects it...
    if cdrom_state.read_buffer.is_empty() {
        if cdrom_state.pausing {
            // Stop reading and raise interrupt.
            cdrom_state.pausing = false;
            cdrom_state.reading = false;
            let stat_value = stat_value(cdrom_state);
            let response = &state.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(state, 2);
            return true;
        }

        if !cdrom_state.reading {
            return false;
        }

        // Make sure FIFO is empty.
        if !state.cdrom.data.is_empty() {
            return true;
        }

        let msf_address_base = cdrom_state.msf_address_base;
        let msf_address_offset = cdrom_state.msf_address_offset;
        let data_block = backend_dispatch::read_sector(cdrom_backend, msf_address_base, msf_address_offset).expect("Tried to read a sector when no backend is available");
        assert_eq!(data_block.len(), 2048);

        cdrom_state.msf_address_offset += 1;
        cdrom_state.read_buffer.extend(&data_block);

        // Raise the interrupt - we have read a sector ok and have some data ready.
        let stat_value = stat_value(cdrom_state);
        let response = &state.cdrom.response;
        response.write_one(stat_value).unwrap();
        raise_irq(state, 1);
    }

    // Check if the CPU is ready for data and send it.
    let request = &state.cdrom.request;
    let load_data = request.register.read_bitfield(REQUEST_BFRD) > 0;
    if load_data {
        let read_buffer = &mut cdrom_state.read_buffer;
        let data = &state.cdrom.data;

        loop {
            if data.is_full() {
                break;
            }

            match read_buffer.pop_front() {
                Some(v) => data.write_one(v).unwrap(),
                None => break,
            }
        }
    }

    true
}
