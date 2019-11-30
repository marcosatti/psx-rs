use std::ptr::NonNull;
use crate::types::fifo::Fifo;
use crate::types::b8_memory_mapper::*;
use crate::types::register::b16_register::B16Register;

pub struct Ctrl {
    pub register: B16Register,
    pub write_latch: bool,
}

impl Ctrl {
    pub fn new() -> Ctrl {
        Ctrl {
            register: B16Register::new(),
            write_latch: false,
        }
    }
}

impl B8MemoryMap for Ctrl {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        B8MemoryMap::read_u16(&mut self.register, offset)
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        if self.write_latch { panic!("Write latch still on"); }
        B8MemoryMap::write_u16(&mut self.register, offset, value).unwrap();
        self.write_latch = true;
        Ok(())
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
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        unsafe {
            if offset != 0 { panic!("Invalid offset"); }
            self.tx_fifo.as_ref().unwrap().as_ref().read_one().map(|v| v as u32).map_err(|_| ReadError::Empty)
        }
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        unsafe {
            if offset != 0 { panic!("Invalid offset"); }
            let value_u8 = value as u8;
            self.tx_fifo.as_ref().unwrap().as_ref().write_one(value_u8).map_err(|_| WriteError::Full)
        }
    }
}
