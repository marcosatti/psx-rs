// use log::debug;
use crate::system::types::State;

pub fn handle_command(state: &mut State, _data: u8) {
    // debug!("PADMC command 0x{:X} received, returning 0xFF in RX FIFO", data);
    state.padmc.rx_fifo.write_one(0xFF).unwrap();
}
