use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use log::warn;
use crate::types::fifo::Fifo;
use crate::types::b8_memory_mapper::*;
use crate::types::register::b16_register::B16Register;

pub struct Ctrl {
    pub register: B16Register,
    pub write_latch: AtomicBool,
}

impl Ctrl {
    pub fn new() -> Ctrl {
        Ctrl {
            register: B16Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for Ctrl {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        //assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
    }
}

pub struct Padmc1040 {
    pub tx_fifo: Option<NonNull<Fifo<u8>>>,
    pub rx_fifo: Option<NonNull<Fifo<u8>>>,
}

impl Padmc1040 {
    pub fn new() -> Padmc1040 {
        Padmc1040 {
            tx_fifo: None,
            rx_fifo: None,
        }
    }
}

impl B8MemoryMap for Padmc1040 {
    fn read_u8(&mut self, _offset: u32) -> ReadResult<u8> {
        unsafe {
            Ok(self.tx_fifo.as_ref().unwrap().as_ref().read_one().unwrap_or_else(|_| {
                warn!("PADMC RX FIFO empty - returning 0xFF");
                0xFF
            }))
        }
    }
    
    fn write_u8(&mut self, _offset: u32, value: u8) -> WriteResult {
        unsafe {
            self.tx_fifo.as_ref().unwrap().as_ref().write_one(value).map_err(|_| WriteError::Full)
        }
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        unsafe {
            assert!(offset == 0, "Invalid offset");
            warn!("PADMC RX FIFO preview reads not properly implemented");
            self.tx_fifo.as_ref().unwrap().as_ref().read_one().map(|v| v as u32).map_err(|_| ReadError::Empty)
        }
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        unsafe {
            assert!(offset == 0, "Invalid offset");
            let value_u8 = value as u8;
            self.tx_fifo.as_ref().unwrap().as_ref().write_one(value_u8).map_err(|_| WriteError::Full)
        }
    }
}
