use crate::system::types::State;
use crate::system::memory::types::*;
use crate::system::cdrom::constants::*;

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
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => unimplemented!(),
        1 => state.cdrom.response.read_one().map_err(|_| ReadErrorKind::Empty),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}

pub fn cdrom1801_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => Ok(state.cdrom.command.write_u8(value)),
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}

fn cdrom1802_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => unimplemented!(),
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}

fn cdrom1802_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => state.cdrom.parameter.write_one(value).map_err(|_| WriteErrorKind::Full),
        1 => Ok(state.cdrom.int_enable.write_u8(value)),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}

fn cdrom1803_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    assert_eq!(offset, 0);
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => Ok(state.cdrom.int_enable.read_u8()),
        1 => Ok(state.cdrom.int_flag.read_u8()),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}

fn cdrom1803_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    assert_eq!(offset, 0);
    let index = state.cdrom.status.read_bitfield(STATUS_INDEX);
    match index {
        0 => Ok(state.cdrom.request.write_u8(value)),
        1 => Ok(state.cdrom.int_flag.write_u8(value)),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!("Index {} does not exist", index),
    }
}
