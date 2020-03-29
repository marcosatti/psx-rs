use crate::types::{
    b8_memory_mapper::*,
    bitfield::Bitfield,
};

#[repr(C)]
#[derive(Copy, Clone)]
union B8Register_ {
    v8: u8,
}

#[derive(Copy, Clone)]
pub struct B8Register {
    value: B8Register_,
}

impl B8Register {
    pub fn new() -> B8Register {
        B8Register {
            value: B8Register_ {
                v8: 0,
            },
        }
    }

    pub fn read_u8(&self) -> u8 {
        unsafe { self.value.v8 }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.value.v8 = value;
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
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        assert!(offset == 0, "Invalid offset");
        Ok(Self::read_u8(self))
    }

    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        Self::write_u8(self, value);
        Ok(())
    }
}
