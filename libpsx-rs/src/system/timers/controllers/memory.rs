use crate::system::types::State;
use crate::system::memory::types::*;
use crate::system::timers::controllers::timer::*;

pub fn count_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).read_u16(offset))
}

pub fn count_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).write_u16(offset, value))
}

pub fn count_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).read_u32())
}

pub fn count_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).write_u32(value))
}

pub fn mode_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    Ok(get_count(state, timer_id).read_u16(offset / 2))
}

pub fn mode_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    Ok(get_mode(state, timer_id).write_u16(offset / 2, value))
}

pub fn mode_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_mode(state, timer_id).read_u32())
}

pub fn mode_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_mode(state, timer_id).write_u32(value))
}

pub fn target_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    Ok(get_target(state, timer_id).read_u16(offset / 2))
}

pub fn target_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    Ok(get_target(state, timer_id).write_u16(offset / 2, value))
}

pub fn target_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_target(state, timer_id).read_u32())
}

pub fn target_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_target(state, timer_id).write_u32(value))
}
