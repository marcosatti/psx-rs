use crate::types::b8_memory_mapper::*;

pub struct B8Memory {
    pub memory: Vec<u8>,
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

    pub fn read_raw(&self, offset: u32) -> &[u8] {
        &self.memory[offset as usize..]
    }

    pub fn write_raw(&mut self, data: &[u8], offset: u32) {
        self.memory[offset as usize..(offset as usize) + data.len()].copy_from_slice(data);
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        self.memory[offset as usize]
    }

    pub fn write_u8(&mut self, offset: u32, value: u8) {
        self.memory[offset as usize] = value;
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        unsafe {
            *((&self.memory[offset as usize] as *const u8) as *const u16)
        }
    }

    pub fn write_u16(&mut self, offset: u32, value: u16) {
        unsafe {
            *((&mut self.memory[offset as usize] as *mut u8) as *mut u16) = value;
        }
    }

    pub fn read_u32(&self, offset: u32) -> u32 {
        unsafe {
            *((&self.memory[offset as usize] as *const u8) as *const u32)
        }
    }

    pub fn write_u32(&mut self, offset: u32, value: u32) {
        unsafe {
            *((&mut self.memory[offset as usize] as *mut u8) as *mut u32) = value;
        }
    }
}

impl B8MemoryMap for B8Memory {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        Ok(Self::read_u8(self, offset))
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        Self::write_u8(self, offset, value);
        Ok(())
    }

    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        Ok(Self::read_u16(self, offset))
    }
    
    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        Self::write_u16(self, offset, value);
        Ok(())
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        Ok(Self::read_u32(self, offset))
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        Self::write_u32(self, offset, value);
        Ok(())
    }
}
