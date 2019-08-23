use spin::Mutex;
use crate::types::queue::Queue;
use crate::types::register::b16_register::B16Register;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::bitfield::Bitfield;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferMode {
    Stop,
    ManualWrite,
    DmaWrite,
    DmaRead,
}

pub struct Fifo {
    pub fifo: Queue<u16>, 
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            fifo: Queue::new(64, "SPU FIFO", false),
        }
    }
}

impl B8MemoryMap for Fifo {
    fn write_u16(&mut self, offset: usize, value: u16) -> WriteResult {
        if offset != 0 { panic!("Invalid offset"); }
        self.fifo.write_one(value).map_err(|_| WriteError::Full)
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
    fn read_u16(&mut self, offset: usize) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: usize, value: u16) -> WriteResult {
        if self.write_latch { panic!("Write latch still on"); }
        B8MemoryMap::write_u16(&mut self.register, offset, value).unwrap();
        self.write_latch = true;
        Ok(())
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
    fn read_u16(&mut self, offset: usize) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: usize, value: u16) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u16(&mut self.register, offset, value).unwrap();
        for i in 0..16 {
            self.write_latch[(offset * 8) + i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }

    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }

    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u32(&mut self.register, offset, value).unwrap();
        for i in 0..32 {
            self.write_latch[i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }
}
