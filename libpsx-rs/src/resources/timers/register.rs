use std::sync::atomic::{AtomicBool, Ordering};
use crate::types::b8_memory_mapper::*;
use crate::types::register::b32_register::B32Register;

pub struct Mode {
    pub register: B32Register,
    pub write_latch: AtomicBool,
    pub read_latch: AtomicBool,
}

impl Mode {
    pub fn new() -> Mode {
        Mode {
            register: B32Register::new(),
            write_latch: AtomicBool::new(false),
            read_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for Mode {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        let result = B8MemoryMap::read_u16(&mut self.register, offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        let result = B8MemoryMap::read_u32(&mut self.register, offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}
