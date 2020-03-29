use crate::types::{
    b8_memory_mapper::*,
    bitfield::Bitfield,
};

#[repr(C)]
#[derive(Copy, Clone)]
union B16Register_ {
    v16: u16,
    v8: [u8; 2],
}

#[derive(Copy, Clone)]
pub struct B16Register {
    value: B16Register_,
}

impl B16Register {
    pub fn new() -> B16Register {
        B16Register {
            value: B16Register_ {
                v16: 0,
            },
        }
    }

    pub fn read_u16(&self) -> u16 {
        unsafe { self.value.v16 }
    }

    pub fn write_u16(&mut self, value: u16) {
        self.value.v16 = value;
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe { self.value.v8[offset as usize] }
    }

    pub fn write_u8(&mut self, offset: u32, value: u8) {
        unsafe {
            self.value.v8[offset as usize] = value;
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
        assert!(offset == 0, "Invalid offset");
        Ok(Self::read_u16(self))
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        Self::write_u16(self, value);
        Ok(())
    }
}
