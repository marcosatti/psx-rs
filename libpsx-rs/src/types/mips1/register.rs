use crate::types::bitfield::Bitfield;
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone)]
union Register_ {
    v32: u32,
    v16: [u16; 2],
    v8: [u8; 4],
}

impl fmt::Debug for Register_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("Register")
                .field("v32", &self.v32)
                .field("v16[0]", &self.v16[0])
                .field("v16[1]", &self.v16[1])
                .field("v8[0]", &self.v8[0])
                .field("v8[1]", &self.v8[1])
                .field("v8[2]", &self.v8[2])
                .field("v8[3]", &self.v8[3])
                .finish()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Register {
    memory: Register_,
}

impl Register {
    pub(crate) fn new() -> Register {
        Register {
            memory: Register_ {
                v32: 0,
            },
        }
    }

    pub(crate) fn read_u8(&self, offset: u32) -> u8 {
        unsafe { self.memory.v8[offset as usize] }
    }

    pub(crate) fn write_u8(&mut self, offset: u32, value: u8) {
        unsafe {
            self.memory.v8[offset as usize] = value;
        }
    }

    pub(crate) fn read_u16(&self, offset: u32) -> u16 {
        unsafe { self.memory.v16[offset as usize] }
    }

    pub(crate) fn write_u16(&mut self, offset: u32, value: u16) {
        unsafe {
            self.memory.v16[offset as usize] = value;
        }
    }

    pub(crate) fn read_u32(&self) -> u32 {
        unsafe { 
            self.memory.v32
        }
    }

    pub(crate) fn write_u32(&mut self, value: u32) {
        self.memory.v32 = value;
    }

    pub(crate) fn read_bitfield(&self, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32())
    }

    pub(crate) fn write_bitfield(&mut self, bitfield: Bitfield, value: u32) {
        self.write_u32(bitfield.insert_into(self.read_u32(), value));
    }
}
