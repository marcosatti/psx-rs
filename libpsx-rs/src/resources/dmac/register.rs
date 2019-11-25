use std::ptr::NonNull;
use parking_lot::Mutex;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::bitfield::Bitfield;
use crate::resources::dmac::*;

pub struct Dicr {
    pub mutex: Mutex<()>, 
    pub register: B32Register,
}

impl Dicr {
    pub fn new() -> Dicr {
        Dicr {
            mutex: Mutex::new(()),
            register: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Dicr {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        let mut register_value = self.register.read_u32()
        register_value = Bitfield::new(0, 6).copy(register_value, value);
        register_value = Bitfield::new(15, 1).copy(register_value, value);
        register_value = Bitfield::new(16, 7).copy(register_value, value);
        register_value = Bitfield::new(23, 1).copy(register_value, value);
        register_value = Bitfield::new(24, 7).acknowledge(register_value, value)
        B8MemoryMap::write_u32(&mut self.register, offset, register_value)
    }
}

pub struct OtcChcr {
    pub register: B32Register,
}

impl OtcChcr {
    pub fn new() -> OtcChcr {
        OtcChcr { 
            register: B32Register::from(0x0000_0002),
        }
    }
}

impl B8MemoryMap for OtcChcr {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.chcr, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        let mut register_value = self.chcr.register.read_u32();
        register_value = CHCR_STARTBUSY.copy(register_value, value);
        register_value = CHCR_STARTTRIGGER.copy(register_value, value);
        register_value = CHCR_BIT30.copy(register_value, value);
        B8MemoryMap::write_u32(&mut self.chcr, offset, register_value)
    }
}
