use crate::system::types::State;
use crate::system::memory::types::*;
use crate::system::timers::controllers::timer::*;

pub fn count_read_u16(state: &State, offset: u32, index: usize) -> ReadResult<u16> {
    Ok(get_count(state, index).read_u16(offset))
}

pub fn count_write_u16(state: &State, offset: u32, value: u16, index: usize) -> WriteResult {
    Ok(get_count(state, index).write_u16(offset, value))
}

pub fn count_read_u32(state: &State, offset: u32, index: usize) -> ReadResult<u32> {
    Ok(get_count(state, index).read_u32(offset))
}

pub fn count_write_u32(state: &State, offset: u32, value: u32, index: usize) -> WriteResult {
    Ok(get_count(state, index).write_u32(offset, value))
}

pub fn mode_read_u16(state: &State, offset: u32, index: usize) -> ReadResult<u16> {
    Ok(get_count(state, index).read_u16(offset))
}

pub fn mode_write_u16(state: &State, offset: u32, value: u16, index: usize) -> WriteResult {
    Ok(get_mode(state, index).write_u16(offset, value))
}

pub fn mode_read_u32(state: &State, offset: u32, index: usize) -> ReadResult<u32> {
    Ok(get_mode(state, index).read_u32(offset))
}

pub fn mode_write_u32(state: &State, offset: u32, value: u32, index: usize) -> WriteResult {
    Ok(get_mode(state, index).write_u32(offset, value))
}

pub fn target_read_u16(state: &State, offset: u32, index: usize) -> ReadResult<u16> {
    Ok(get_target(state, index).read_u16(offset))
}

pub fn target_write_u16(state: &State, offset: u32, value: u16, index: usize) -> WriteResult {
    Ok(get_target(state, index).write_u16(offset, value))
}

pub fn target_read_u32(state: &State, offset: u32, index: usize) -> ReadResult<u32> {
    Ok(get_target(state, index).read_u32(offset))
}

pub fn target_write_u32(state: &State, offset: u32, value: u32, index: usize) -> WriteResult {
    Ok(get_target(state, index).write_u32(offset, value))
}
