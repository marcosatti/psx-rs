use crate::system::types::State;
use crate::system::bus::types::*;

pub fn stat_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.intc.stat.read_u16(offset / 2))
}

pub fn stat_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.intc.stat.write_u16(offset / 2, value))
}

pub fn stat_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.intc.stat.read_u32())
}

pub fn stat_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.intc.stat.write_u32(value))
}

pub fn mask_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.intc.mask.read_u16(offset / 2))
}

pub fn mask_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.intc.mask.write_u16(offset / 2, value))
}

pub fn mask_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.intc.mask.read_u32())
}

pub fn mask_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.intc.mask.write_u32(value))
}
