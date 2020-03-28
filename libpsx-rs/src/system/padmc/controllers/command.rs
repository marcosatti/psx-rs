//use log::debug;
use crate::system::Resources;

pub fn handle_command(resources: &mut Resources, _data: u8) {
    //debug!("PADMC command 0x{:X} received, returning 0xFF in RX FIFO", data);
    resources.padmc.rx_fifo.write_one(0xFF).unwrap();
}
