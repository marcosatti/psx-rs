use crate::system::types::State;
use crate::system::memory::types::*;

fn gpu1810_read_u32(state: &State, offset: u32) -> u32 {
    assert_eq!(offset, 0);
    Ok(state.gpu.read.read_one().unwrap_or_else(|_| {
        log::warn!("GPUREAD is empty - returning 0xFFFF_FFFF");
        0xFFFF_FFFF
    }))
}

fn gpu1810_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.gpu.gp0.write_one(value).map_err(|_| WriteErrorKind::Full)
}

fn gpu1814_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.gpu.stat.read_u32())
}

fn gpu1814_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    state.gpu.gp1.write_one(value).map_err(|_| WriteErrorKind::Full)
}
