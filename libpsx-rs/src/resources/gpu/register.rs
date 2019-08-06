use log::debug;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::queue::Queue;

pub struct Gpu1810 {
    pub gp0: Queue<u32>,
    pub read: Queue<u32>, 
}

impl Gpu1810 {
    pub fn new() -> Gpu1810 {
        Gpu1810 {
            gp0: Queue::new(),
            read: Queue::new(),
        }
    }
}

impl B8MemoryMap for Gpu1810 {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        if offset != 0 { panic!("Invalid offset"); }
        
        Ok(self.read.read_one().unwrap_or_else(|_| {
            debug!("GPUREAD is empty - returning 0");
            0
        }))
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        if offset != 0 { panic!("Invalid offset"); }
        self.gp0.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct Gpu1814 {
    pub gp1: Queue<u32>, 
    pub stat: B32Register,
}

impl Gpu1814 {
    pub fn new() -> Gpu1814 {
        Gpu1814 {
            gp1: Queue::new(), // Not really a FIFO(?), but emulator needs to buffer commands.
            stat: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Gpu1814 {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.stat, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        if offset != 0 { panic!("Invalid offset"); }
        self.gp1.write_one(value)
    }
}
