use crate::system::{memory::types::{ReadResult, WriteResult}, types::State};

pub fn bus_read_u8(state: &State, address: u32) -> ReadResult<u8> {
    unimplemented!();
}

pub fn bus_write_u8(state: &State, address: u32, value: u8) -> WriteResult {
    unimplemented!();
}

pub fn bus_read_u16(state: &State, address: u32) -> ReadResult<u16> {
    unimplemented!();
}

pub fn bus_write_u16(state: &State, address: u32, value: u16) -> WriteResult {
    unimplemented!();
}

pub fn bus_read_u32(state: &State, address: u32) -> ReadResult<u32> {
    unimplemented!();
}

pub fn bus_write_u32(state: &State, address: u32, value: u32) -> WriteResult {
    unimplemented!();
}
