use crate::system::types::State;
use crate::system::memory::types::*;
use crate::system::dmac::controllers::channel::*;

pub fn madr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_madr(state, channel_id).read_u32())
}

pub fn madr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_madr(state, channel_id).write_u32(value))
}

pub fn bcr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_bcr(state, channel_id).read_u32())
}

pub fn bcr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_bcr(state, channel_id).write_u32(value))
}

pub fn chcr_read_u32(state: &State, offset: u32, channel_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_chcr(state, channel_id).read_u32())
}

pub fn chcr_write_u32(state: &State, offset: u32, value: u32, channel_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_chcr(state, channel_id).write_u32(value))
}

pub fn dpcr_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.dmac.dpcr.read_u32())
}

pub fn dpcr_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.dmac.dpcr.write_u32(value))
}

pub fn dicr_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.dmac.dicr.read_u32())
}

pub fn dicr_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.dmac.dicr.write_u32(value))
}
