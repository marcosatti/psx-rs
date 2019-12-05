use crate::resources::Resources;

pub fn handle_command(resources: &mut Resources, data: u8) {
    log::debug!("PADMC command 0x{:X} received, returning 0xFF in RX FIFO", data);
    resources.padmc.rx_fifo.write_one(0xFF).unwrap();
}
