use crate::system::{
    bus::types::*,
    types::State,
};
use crate::system::cdrom::constants::*;

// Emulation note:
// BIOS accesses ports at 0x1F80_1801 / 2 / 3 immediately after writing to the status register (no wait cycles or acknowledgment).

pub fn status_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    Ok(state.cdrom.status.read_u8())
}

pub fn status_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.cdrom.status.write_u8(value))
}

pub fn cdrom1801_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0..=3 => state.cdrom.response.read_one().map_err(|_| ReadErrorKind::Empty),
        _ => unreachable!(),
    }
}

pub fn cdrom1801_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.command.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        1..=3 => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn cdrom1802_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0..=3 => state.cdrom.data.read_one().map_err(|_| ReadErrorKind::Empty),
        _ => unreachable!(),
    }
}

pub fn cdrom1802_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.parameter.write_one(value).map_err(|_| WriteErrorKind::Full),
        1 => Ok(state.cdrom.interrupt_enable.write_u8(value)),
        2..=3 => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn cdrom1803_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => Ok(state.cdrom.interrupt_enable.read_u8()),
        1 => state.cdrom.interrupt_flag.read_u8().map_err(|_| ReadErrorKind::NotReady),
        2 => Ok(state.cdrom.interrupt_enable.read_u8()),
        3 => state.cdrom.interrupt_flag.read_u8().map_err(|_| ReadErrorKind::NotReady),
        _ => unreachable!(),
    }
}

pub fn cdrom1803_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    match state.cdrom.status.read_bitfield(STATUS_INDEX) {
        0 => state.cdrom.request.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        1 => state.cdrom.interrupt_flag.write_u8(value).map_err(|_| WriteErrorKind::NotReady),
        2..=3 => unimplemented!(),
        _ => unreachable!(),
    }
}
