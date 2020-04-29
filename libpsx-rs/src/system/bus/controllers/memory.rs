use crate::system::{memory::types::{ReadResult, WriteResult}, types::State};

pub fn bus_read_u8(_state: &State, _address: u32) -> ReadResult<u8> {
    unimplemented!();
}

pub fn bus_write_u8(_state: &State, _address: u32, _value: u8) -> WriteResult {
    unimplemented!();
}

pub fn bus_read_u16(_state: &State, _address: u32) -> ReadResult<u16> {
    unimplemented!();
}

pub fn bus_write_u16(_state: &State, _address: u32, _value: u16) -> WriteResult {
    unimplemented!();
}

pub fn bus_read_u32(_state: &State, _address: u32) -> ReadResult<u32> {
    unimplemented!();
}

pub fn bus_write_u32(_state: &State, _address: u32, _value: u32) -> WriteResult {
    unimplemented!();
}
