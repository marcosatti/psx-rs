use crate::types::{
    b8_memory_mapper::*,
    bitfield::Bitfield,
};

#[repr(C)]
#[derive(Copy, Clone)]
union B32Register_ {
    v32: u32,
    v16: [u16; 2],
    v8: [u8; 4],
}

#[derive(Copy, Clone)]
pub struct B32Register {
    value: B32Register_,
}

impl B32Register {
    pub fn new() -> B32Register {
        B32Register {
            value: B32Register_ {
                v32: 0,
            },
        }
    }

    pub fn from(value: u32) -> B32Register {
        B32Register {
            value: B32Register_ {
                v32: value,
            },
        }
    }

    pub fn read_u32(&self) -> u32 {
        unsafe { self.value.v32 }
    }

    pub fn write_u32(&mut self, value: u32) {
        self.value.v32 = value;
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        unsafe { self.value.v16[offset as usize] }
    }

    pub fn write_u16(&mut self, offset: u32, value: u16) {
        unsafe {
            self.value.v16[offset as usize] = value;
        }
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe { self.value.v8[offset as usize] }
    }

    pub fn write_u8(&mut self, offset: u32, value: u8) {
        unsafe {
            self.value.v8[offset as usize] = value;
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
        assert!(offset % 2 == 0, "Non aligned offset");
        Ok(Self::read_u16(self, offset / 2))
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(offset % 2 == 0, "Non aligned offset");
        Self::write_u16(self, offset / 2, value);
        Ok(())
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        assert!(offset == 0, "Invalid offset");
        Ok(Self::read_u32(self))
    }

    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        Self::write_u32(self, value);
        Ok(())
    }
}
