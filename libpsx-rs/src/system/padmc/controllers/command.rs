use crate::system::types::State;

pub fn handle_command(state: &State, _data: u8) {
    // log::debug!("PADMC command 0x{:X} received, returning 0xFF in RX FIFO", data);
    state.padmc.rx_fifo.write_one(0xFF).unwrap();
}
