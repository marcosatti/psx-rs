use crate::system::types::State;
use crate::system::memory::types::*;

pub fn voice_key_on_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_key_on.read_u16(offset / 2))
}

pub fn voice_key_on_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_key_on.write_u16(offset / 2, value))
}

pub fn voice_key_on_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_key_on.read_u32())
}

pub fn voice_key_on_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_key_on.write_u32(value))
}

pub fn voice_key_off_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.spu.voice_key_off.read_u16(offset / 2))
}

pub fn voice_key_off_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.spu.voice_key_off.write_u16(offset / 2, value))
}

pub fn voice_key_off_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_key_off.read_u32())
}

pub fn voice_key_off_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.voice_key_off.write_u32(value))
}

pub fn data_transfer_address_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    assert_eq!(offset, 0);
    Ok(state.spu.data_transfer_address.read_u16())
}

pub fn data_transfer_address_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    Ok(state.spu.data_transfer_address.write_u16(value))
}

pub fn data_fifo_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    assert_eq!(offset, 0);
    state.spu.data_fifo.write_one(value).map_err(|_| WriteErrorKind::Full)
}
