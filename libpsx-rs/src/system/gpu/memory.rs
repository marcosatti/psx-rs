use crate::{
    system::{
        bus::types::*,
        gpu::constants::*,
        types::State,
    },
    utilities::bool_to_flag,
};

pub(crate) fn gpu1810_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);

    if state.gpu.gp1_command_pending.load() {
        return Err(ReadErrorKind::NotReady);
    }

    Ok(state.gpu.read.read_one().unwrap_or_else(|_| {
        log::warn!("GPUREAD is empty - returning 0xFFFF_FFFF");
        0xFFFF_FFFF
    }))
}

pub(crate) fn gpu1810_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.gpu.gp0.write_one(value).map_err(|_| WriteErrorKind::Full)
}

pub(crate) fn gpu1814_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    let mut value = state.gpu.stat.read_u32();

    // TODO: not properly implemented most likely...
    // value = STAT_DMA_REQUEST.insert_into(value, 1);
    let gp0_is_full = state.gpu.gp0.is_full();
    value = STAT_RECV_CMD.insert_into(value, !bool_to_flag(gp0_is_full));
    value = STAT_SEND_VRAM.insert_into(value, !bool_to_flag(state.gpu.read.is_empty()));
    value = STAT_RECV_DMA.insert_into(value, !bool_to_flag(gp0_is_full));

    Ok(value)
}

pub(crate) fn gpu1814_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);

    if state.gpu.gp1_command_pending.load() {
        return Err(WriteErrorKind::NotReady);
    }

    state.gpu.gp1_command_pending.store(true);    
    Ok(state.gpu.gp1.write_u32(value))
}
