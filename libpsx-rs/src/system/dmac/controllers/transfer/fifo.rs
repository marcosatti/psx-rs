use crate::system::{
    dmac::constants::*,
    types::State,
};

pub(crate) fn pop_channel_data(state: &State, channel_id: usize, current_address: u32, last_transfer: bool) -> Result<Option<u32>, String> {
    let result = match channel_id {
        2 => state.gpu.read.read_one(),
        3 => {
            let fifo = &state.cdrom.data;

            if fifo.read_available() < 4 {
                return Ok(None);
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
        _ => return Err(format!("Unhandled DMAC channel pop {}", channel_id)),
    };

    Ok(result.ok())
}

pub(crate) fn push_channel_data(state: &State, channel_id: usize, value: u32) -> Result<Option<()>, String> {
    let result = match channel_id {
        2 => state.gpu.gp0.write_one(value),
        6 => return Err("Channel 6 cannot recieve data (OTC)".into()),
        _ => return Err(format!("Unhandled DMAC channel push {}", channel_id)),
    };

    Ok(result.ok())
}
