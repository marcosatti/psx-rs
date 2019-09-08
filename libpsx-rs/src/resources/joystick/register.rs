use std::ptr::NonNull;
use crate::types::fifo::Fifo;
use crate::types::b8_memory_mapper::*;

pub struct Joystick1040 {
    pub tx_fifo: Option<NonNull<Fifo<u8>>>,
    pub rx_fifo: Option<NonNull<Fifo<u8>>>,
}

impl Joystick1040 {
    pub fn new() -> Joystick1040 {
        Joystick1040 {
            tx_fifo: None,
            rx_fifo: None,
        }
    }
}

impl B8MemoryMap for Joystick1040 {
    fn read_u32(&mut self, offset: usize) -> ReadResult<u32> {
        unsafe {
            if offset != 0 { panic!("Invalid offset"); }
            self.tx_fifo.as_ref().unwrap().as_ref().read_one().map(|v| v as u32).map_err(|_| ReadError::Empty)
        }
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) -> WriteResult {
        unsafe {
            if offset != 0 { panic!("Invalid offset"); }
            let value_u8 = value as u8;
            self.tx_fifo.as_ref().unwrap().as_ref().write_one(value_u8).map_err(|_| WriteError::Full)
        }
    }
}
