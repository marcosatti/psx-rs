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
        },
        types::State,
    },
};

pub fn handle_read(state: &mut State, cdrom_backend: &CdromBackend) -> bool {
    // Buffer some data first (INT1 means ready to send data?).
    // Do we always want to send data if we have read a sector regardless of the current reading status? Seems like the
    // BIOS expects it...
    if state.cdrom.read_buffer.is_empty() {
        if state.cdrom.pausing {
            // Stop reading and raise interrupt.
            state.cdrom.pausing = false;
            state.cdrom.reading = false;
            let stat_value = stat_value(state);
            let response = &mut state.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(state, 2);
            return true;
        }

        if !state.cdrom.reading {
            return false;
        }

        // Make sure FIFO is empty.
        if !state.cdrom.data.is_empty() {
            return true;
        }

        let data_block = backend_dispatch::read_sector(cdrom_backend, state.cdrom.lba_address)
            .expect("Tried to read a sector when no backend is available");
        assert_eq!(data_block.len(), 2048);

        state.cdrom.lba_address += 1;
        state.cdrom.read_buffer.extend(&data_block);

        // Raise the interrupt - we have read a sector ok and have some data ready.
        let stat_value = stat_value(state);
        let response = &mut state.cdrom.response;
        response.write_one(stat_value).unwrap();
        raise_irq(state, 1);
    }

    // Check if the CPU is ready for data and send it.
    let request = &mut state.cdrom.request;
    let load_data = request.register.read_bitfield(REQUEST_BFRD) > 0;
    if load_data {
        let read_buffer = &mut state.cdrom.read_buffer;
        let data = &mut state.cdrom.data;

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
