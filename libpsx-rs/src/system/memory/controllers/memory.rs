use crate::system::types::State;
use crate::system::memory::types::*;

pub fn main_memory_read_u8(state: &State, offset: u32) -> ReadResult<u8> {
    Ok(state.memory.main_memory.read_u8(offset))
}

pub fn main_memory_write_u8(state: &State, offset: u32, value: u8) -> WriteResult {
    Ok(state.memory.main_memory.write_u8(offset, value))
}

pub fn main_memory_read_u16(state: &State, offset: u32) -> ReadResult<u16> {
    Ok(state.memory.main_memory.read_u16(offset))
}

pub fn main_memory_write_u16(state: &State, offset: u32, value: u16) -> WriteResult {
    Ok(state.memory.main_memory.write_u16(offset, value))
}

pub fn main_memory_read_u32(state: &State, offset: u32) -> ReadResult<u32> {
    Ok(state.memory.main_memory.read_u32(offset))
}

pub fn main_memory_write_u32(state: &State, offset: u32, value: u32) -> WriteResult {
    Ok(state.memory.main_memory.write_u32(offset, value))
}
