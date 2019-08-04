use spin::Mutex;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::bitfield::Bitfield;

pub struct Stat {
    pub mutex: Mutex<()>, 
    pub register: B32Register,
}

impl Stat {
    pub fn new() -> Stat {
        Stat {
            mutex: Mutex::new(()),
            register: B32Register::new(),
        }
    }

    pub fn set_irq(&mut self, bitfield: Bitfield) {
        let _lock = self.mutex.lock();
        self.register.write_bitfield(bitfield, 1);
    }
}

impl B8MemoryMap for Stat {
    fn read_u16(&mut self, offset: usize) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }
    
    fn write_u16(&mut self, offset: usize, value: u16) -> WriteResult {
        let _lock = self.mutex.lock();
        let value = value & self.register.read_u16(offset);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
    }

    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        let value = value & self.register.read_u32();
        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}
