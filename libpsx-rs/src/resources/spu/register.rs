use spin::Mutex;
use std::collections::VecDeque;
use crate::types::register::b16_register::B16Register;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

pub struct Fifo {
    pub lock: Mutex<()>,
    pub fifo: VecDeque<u16>, 
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            lock: Mutex::new(()),
            fifo: VecDeque::with_capacity(1024),
        }
    }
}

impl B8MemoryMap for Fifo {
    fn write_u16(&mut self, offset: usize, value: u16) {
        if offset != 0 { panic!("Invalid offset"); }
        let _lock = self.lock.lock();
        self.fifo.push_back(value);
    }
}

pub struct TransferAddress {
    pub register: B16Register,
    pub write_latch: bool,
}

impl TransferAddress {
    pub fn new() -> TransferAddress {
        TransferAddress {
            register: B16Register::new(),
            write_latch: false,
        }
    }
}

impl B8MemoryMap for TransferAddress {
    fn read_u16(&mut self, offset: usize) -> u16 {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: usize, value: u16) {
        if self.write_latch { panic!("Write latch still on"); }
        B8MemoryMap::write_u16(&mut self.register, offset, value);
        self.write_latch = true;
    }
}

pub struct VoiceKey {
    pub register: B32Register,
    pub write_latch: [bool; 32],
    pub mutex: Mutex<()>,
}

impl VoiceKey {
    pub fn new() -> VoiceKey {
        VoiceKey {
            register: B32Register::new(),
            write_latch: [false; 32],
            mutex: Mutex::new(()),
        }
    }
}

impl B8MemoryMap for VoiceKey {
    fn read_u16(&mut self, offset: usize) -> u16 {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: usize, value: u16) {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u16(&mut self.register, offset, value);
        for i in 0..16 {
            self.write_latch[(offset * 8) + i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
    }

    fn read_u32(&mut self, offset: usize) -> u32 {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }

    fn write_u32(&mut self, offset: usize, value: u32) {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u32(&mut self.register, offset, value);
        for i in 0..32 {
            self.write_latch[i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
    }
}
