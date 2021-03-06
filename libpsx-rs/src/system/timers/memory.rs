use crate::system::{
    bus::types::*,
    timers::controllers::timer::*,
    types::State,
};

pub(crate) fn count_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).read_u16(offset))
}

pub(crate) fn count_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).write_u16(offset, value))
}

pub(crate) fn count_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).read_u32())
}

pub(crate) fn count_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_count(state, timer_id).write_u32(value))
}

pub(crate) fn mode_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    get_mode(state, timer_id).read_u16(offset / 2).map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn mode_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    get_mode(state, timer_id).write_u16(offset / 2, value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn mode_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    get_mode(state, timer_id).read_u32().map_err(|_| ReadErrorKind::NotReady)
}

pub(crate) fn mode_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    get_mode(state, timer_id).write_u32(value).map_err(|_| WriteErrorKind::NotReady)
}

pub(crate) fn target_read_u16(state: &State, offset: u32, timer_id: usize) -> ReadResult<u16> {
    Ok(get_target(state, timer_id).read_u16(offset / 2))
}

pub(crate) fn target_write_u16(state: &State, offset: u32, value: u16, timer_id: usize) -> WriteResult {
    Ok(get_target(state, timer_id).write_u16(offset / 2, value))
}

pub(crate) fn target_read_u32(state: &State, offset: u32, timer_id: usize) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(get_target(state, timer_id).read_u32())
}

pub(crate) fn target_write_u32(state: &State, offset: u32, value: u32, timer_id: usize) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(get_target(state, timer_id).write_u32(value))
}
