use std::sync::atomic::{AtomicBool, Ordering};
use parking_lot::Mutex;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;
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

pub struct DataFifo {
    pub fifo: Fifo<u16>, 
}

impl DataFifo {
    pub fn new() -> DataFifo {
        DataFifo {
            fifo: Fifo::new(64, Some(DebugState::new("SPU FIFO", false, false))),
        }
    }
}

impl B8MemoryMap for DataFifo {
    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.fifo.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct TransferAddress {
    pub register: B16Register,
    pub write_latch: AtomicBool,
}

impl TransferAddress {
    pub fn new() -> TransferAddress {
        TransferAddress {
            register: B16Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for TransferAddress {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
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
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u16(&mut self.register, offset, value).unwrap();
        for i in 0..16 {
            self.write_latch[((offset * 8) + (i as u32)) as usize] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.register, offset)
    }

    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        let _lock = self.mutex.lock();
        B8MemoryMap::write_u32(&mut self.register, offset, value).unwrap();
        for i in 0..32 {
            self.write_latch[i] = Bitfield::new(i, 1).extract_from(value) != 0;
        }
        Ok(())
    }
}
