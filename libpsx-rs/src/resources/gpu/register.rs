use log::debug;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;

pub struct Gpu1810 {
    pub gp0: Fifo<u32>,
    pub read: Fifo<u32>, 
}

impl Gpu1810 {
    pub fn new() -> Gpu1810 {
        Gpu1810 {
            gp0: Fifo::new(64, Some(DebugState::new("GPU GP0", true, true))),
            read: Fifo::new(64, Some(DebugState::new("GPU READ", false, false))),
        }
    }
}

impl B8MemoryMap for Gpu1810 {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        assert!(offset == 0, "Invalid offset");
        
        Ok(self.read.read_one().unwrap_or_else(|_| {
            debug!("GPUREAD is empty - returning 0xFFFF_FFFF");
            0xFFFF_FFFF
        }))
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.gp0.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct Gpu1814 {
    pub gp1: Fifo<u32>, 
    pub stat: B32Register,
}

impl Gpu1814 {
    pub fn new() -> Gpu1814 {
        Gpu1814 {
            gp1: Fifo::new(64, Some(DebugState::new("GPU GP1", true, true))), // Not really a FIFO(?), but emulator needs to buffer commands.
            stat: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Gpu1814 {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.stat, offset)
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.gp1.write_one(value).map_err(|_| WriteError::Full)
    }
}
