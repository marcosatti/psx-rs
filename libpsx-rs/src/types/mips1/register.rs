use crate::{
    types::bitfield::Bitfield,
    utilities::primitive::*,
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct Register {
    memory: u32,
}

impl Register {
    pub(crate) fn new() -> Register {
        Register {
            memory: 0,
        }
    }

    pub(crate) fn read_u8(&self, offset: usize) -> u8 {
        u32::extract_u8_le(self.memory, offset)
    }

    pub(crate) fn write_u8(&mut self, offset: usize, value: u8) {
        self.memory = u32::insert_u8_le(self.memory, offset, value);
    }

    pub(crate) fn read_u16(&self, offset: usize) -> u16 {
        u32::extract_u16_le(self.memory, offset)
    }

    pub(crate) fn write_u16(&mut self, offset: usize, value: u16) {
        self.memory = u32::insert_u16_le(self.memory, offset, value);
    }

    pub(crate) fn read_u32(&self) -> u32 {
        self.memory
    }

    pub(crate) fn write_u32(&mut self, value: u32) {
        self.memory = value;
    }

    pub(crate) fn read_bitfield(&self, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32())
    }

    pub(crate) fn write_bitfield(&mut self, bitfield: Bitfield, value: u32) {
        self.write_u32(bitfield.insert_into(self.read_u32(), value));
    }
}
