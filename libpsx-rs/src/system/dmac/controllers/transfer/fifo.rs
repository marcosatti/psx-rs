use crate::system::{
    dmac::constants::*,
    types::State,
};

pub fn pop_channel_data(state: &State, channel_id: usize, current_address: u32, last_transfer: bool) -> Result<u32, ()> {
    match channel_id {
        2 => state.gpu.read.read_one(),
        3 => {
            let fifo = &state.cdrom.data;

            if fifo.read_available() < 4 {
                return Err(());
            }

            let result1 = fifo.read_one().unwrap();
            let result2 = fifo.read_one().unwrap();
            let result3 = fifo.read_one().unwrap();
            let result4 = fifo.read_one().unwrap();

            Ok(u32::from_le_bytes([result1, result2, result3, result4]))
        },
        6 => {
            Ok(if !last_transfer {
                (current_address - DATA_SIZE) & 0x00FF_FFFF
            } else {
                0x00FF_FFFF
            })
        },
        _ => unimplemented!("Unhandled DMAC channel pop {}", channel_id),
    }
}

pub fn push_channel_data(state: &State, channel_id: usize, value: u32) -> Result<(), ()> {
    match channel_id {
        2 => state.gpu.gp0.write_one(value),
        6 => panic!("Channel 6 cannot recieve data (OTC)"),
        _ => unimplemented!("Unhandled DMAC channel push {}", channel_id),
    }
}
