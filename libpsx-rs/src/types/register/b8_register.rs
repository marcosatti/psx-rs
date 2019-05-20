use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;

#[repr(C)]
pub union B8Register_ {
    pub v8: u8,
}

pub struct B8Register {
    pub value: B8Register_,
    pub read_only: bool,
}

impl B8Register {
    pub fn new() -> B8Register {
        B8Register { 
            value: B8Register_ { v8: 0 },
            read_only: false,
        }
    }

    pub fn read_only(value: u8) -> B8Register {
        B8Register { 
            value: B8Register_ { v8: value },
            read_only: true,
        }
    }

    pub fn read_u8(&self) -> u8 {
        unsafe { self.value.v8 }
    }

    pub fn write_u8(&mut self, value: u8) {
        if !self.read_only {
            self.value.v8 = value;
        }
    }
    
    pub fn read_bitfield(&self, bitfield: Bitfield) -> u8 {
        bitfield.extract_from(self.read_u8())
    }

    pub fn write_bitfield(&mut self, bitfield: Bitfield, value: u8) {
        let current = B8Register::read_u8(self);
        self.write_u8(bitfield.insert_into(current, value));
    }
}

impl B8MemoryMap for B8Register {
    fn read_u8(&mut self, offset: usize) -> u8 {
        if offset != 0 { panic!("Invalid offset"); }
        Self::read_u8(self)
    }
    
    fn write_u8(&mut self, offset: usize, value: u8) {
        if offset != 0 { panic!("Invalid offset"); }
        Self::write_u8(self, value);
    }
}