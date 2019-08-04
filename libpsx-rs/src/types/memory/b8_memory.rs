use crate::types::b8_memory_mapper::*;

pub struct B8Memory {
    memory: Vec<u8>,
}

impl B8Memory {
    pub fn new(size: usize) -> B8Memory {
        B8Memory {
            memory: vec![0; size],
        }
    }

    pub fn new_initialized(size: usize, value: u8) -> B8Memory {
        B8Memory {
            memory: vec![value; size],
        }
    }

    pub fn read_raw(&self, offset: usize) -> &[u8] {
        &self.memory[offset..]
    }

    pub fn write_raw(&mut self, data: &[u8], offset: usize) {
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }

    pub fn read_u8(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn write_u8(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    pub fn read_u16(&self, index: usize) -> u16 {
        unsafe {
            *((&self.memory[index] as *const u8) as *const u16)
        }
    }

    pub fn write_u16(&mut self, index: usize, value: u16) {
        unsafe {
            *((&mut self.memory[index] as *mut u8) as *mut u16) = value;
        }
    }

    pub fn read_u32(&self, index: usize) -> u32 {
        unsafe {
            *((&self.memory[index] as *const u8) as *const u32)
        }
    }

    pub fn write_u32(&mut self, index: usize, value: u32) {
        unsafe {
            *((&mut self.memory[index] as *mut u8) as *mut u32) = value;
        }
    }
}

impl B8MemoryMap for B8Memory {
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        Ok(Self::read_u8(self, offset))
    }
    
    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        Self::write_u8(self, offset, value);
        Ok(())
    }

    fn read_u16(&mut self, offset: usize) -> ReadResult<u16> {
        Ok(Self::read_u16(self, offset))
    }
    
    fn write_u16(&mut self, offset: usize, value: u16) -> WriteResult {
        Self::write_u16(self, offset, value);
        Ok(())
    }

    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        Ok(Self::read_u32(self, offset))
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        Self::write_u32(self, offset, value);
        Ok(())
    }
}
