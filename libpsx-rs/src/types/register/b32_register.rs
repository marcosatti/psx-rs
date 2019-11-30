use crate::types::bitfield::Bitfield;
use crate::types::b8_memory_mapper::*;

#[repr(C)]
pub union B32Register_ {
    pub v32: u32,
    pub v16: [u16; 2],
    pub v8: [u8; 4],
}

pub struct B32Register {
    pub value: B32Register_,
    pub read_only: bool,
}

impl B32Register {
    pub fn new() -> B32Register {
        B32Register { 
            value: B32Register_ { v32: 0 },
            read_only: false
        }
    }

    pub fn from(value: u32) -> B32Register {
        B32Register { 
            value: B32Register_ { v32: value },
            read_only: false
        }
    }

    pub fn read_only(value: u32) -> B32Register {
        B32Register {
            value: B32Register_ { v32: value },
            read_only: true,
        }
    }
    
    pub fn read_u32(&self) -> u32 {
        unsafe { self.value.v32 }
    } 

    pub fn write_u32(&mut self, value: u32) {
        if !self.read_only {
            self.value.v32 = value;
        }
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        unsafe { self.value.v16[offset as usize] }
    }

    pub fn write_u16(&mut self, offset: u32, value: u16) {
        if !self.read_only {
            unsafe { self.value.v16[offset as usize] = value; }
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

    pub fn read_bitfield(&self, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32())
    }

    pub fn write_bitfield(&mut self, bitfield: Bitfield, value: u32) {
        let current = B32Register::read_u32(self);
        self.write_u32(bitfield.insert_into(current, value));
    }
}

impl B8MemoryMap for B32Register {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        Ok(Self::read_u8(self, offset))
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        Self::write_u8(self, offset, value);
        Ok(())
    }

    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        if offset % 2 != 0 { panic!("Non aligned offset"); }
        Ok(Self::read_u16(self, offset / 2))
    }
    
    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        if offset % 2 != 0 { panic!("Non aligned offset"); }
        Self::write_u16(self, offset / 2, value);
        Ok(())
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        if offset != 0 { panic!("Invalid offset"); }
        Ok(Self::read_u32(self))
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        if offset != 0 { panic!("Invalid offset"); }
        Self::write_u32(self, value);
        Ok(())
    }
}
