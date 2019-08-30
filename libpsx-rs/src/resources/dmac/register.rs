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
        let ack_bitfield = Bitfield::new(24, 7);
        let keep_value = value & (!ack_bitfield.shifted_mask::<u32>());
        let ack_register_value = self.register.read_u32() & ack_bitfield.shifted_mask::<u32>();
        let ack_value_value = (!value) & ack_bitfield.shifted_mask::<u32>();
        let value = keep_value | (ack_register_value & ack_value_value);
        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}

pub struct Chcr {
    pub register: B32Register,
    pub bus_locked: Option<NonNull<bool>>,
}

impl Chcr {
    pub fn new() -> Chcr {
        Chcr {
            register: B32Register::new(),
            bus_locked: None,
        }
    }
}

impl B8MemoryMap for Chcr {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        if CHCR_STARTBUSY.extract_from(value) != 0 {
            unsafe { *self.bus_locked.unwrap().as_mut() = true; }
        }

        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}

pub struct OtcChcr {
    pub chcr: Chcr,
}

impl OtcChcr {
    pub fn new() -> OtcChcr {
        let mut r = OtcChcr { chcr: Chcr::new() };
        r.chcr.register = B32Register::from(0x0000_0002);
        r
    }
}

impl B8MemoryMap for OtcChcr {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.chcr, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        let register_value = self.chcr.register.read_u32();
        CHCR_STARTBUSY.insert_into(register_value, value);
        CHCR_STARTTRIGGER.insert_into(register_value, value);
        CHCR_BIT30.insert_into(register_value, value);
        B8MemoryMap::write_u32(&mut self.chcr, offset, value)
    }
}
