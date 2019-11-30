use crate::types::b8_memory_mapper::*;
use crate::types::bitfield::Bitfield;

#[repr(C)]
pub union B16Register_ {
    pub v16: u16,
    pub v8: [u8; 2],
}

pub struct B16Register {
    pub value: B16Register_,
    pub read_only: bool,
}

impl B16Register {
    pub fn new() -> B16Register {
        B16Register { 
            value: B16Register_ { v16: 0 },
            read_only: false,
        }
    }

    pub fn read_only(value: u16) -> B16Register {
        B16Register { 
            value: B16Register_ { v16: value },
            read_only: true,
        }
    }

    pub fn read_u16(&self) -> u16 {
        unsafe { self.value.v16 }
    }

    pub fn write_u16(&mut self, value: u16) {
        if !self.read_only {
            self.value.v16 = value;
        }
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe { self.value.v8[offset as usize] }
    }

    pub fn write_u8(&mut self, offset: u32, value: u8) {
        if !self.read_only {
            unsafe { self.value.v8[offset as usize] = value; }
        }
    }

    pub fn read_bitfield(&self, bitfield: Bitfield) -> u16 {
        bitfield.extract_from(self.read_u16())
    }

    pub fn write_bitfield(&mut self, bitfield: Bitfield, value: u16) {
        let current = B16Register::read_u16(self);
        self.write_u16(bitfield.insert_into(current, value));
    }
}

impl B8MemoryMap for B16Register {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        Ok(Self::read_u8(self, offset))
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        Self::write_u8(self, offset, value);
        Ok(())
    }

    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        if offset != 0 { panic!("Invalid offset"); }
        Ok(Self::read_u16(self))
    }
    
    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        if offset != 0 { panic!("Invalid offset"); }
        Self::write_u16(self, value);
        Ok(())
    }
}
