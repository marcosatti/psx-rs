use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use log::warn;
use crate::types::register::b32_register::B32Register;
use crate::types::register::b16_register::B16Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;
use crate::system::types::State as SystemState;
use crate::types::b8_memory_mapper::*;

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
            Ok(self.rx_fifo.as_ref().unwrap().as_ref().read_one().unwrap_or_else(|_| {
                //warn!("PADMC RX FIFO empty - returning 0xFF");
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
            warn!("PADMC RX FIFO u32 preview reads not properly implemented");
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

pub struct State {
    pub rx_fifo: Fifo<u8>,
    pub tx_fifo: Fifo<u8>,
    pub stat: B32Register,
    pub mode: B16Register,
    pub ctrl: Ctrl,
    pub baud_reload: B16Register,
    pub padmc1040: Padmc1040,
}

impl State {
    pub fn new() -> State {
        State {
            rx_fifo: Fifo::new(16, Some(DebugState::new("PADMC RX", true, true))),
            tx_fifo: Fifo::new(16, Some(DebugState::new("PADMC TX", true, true))),
            stat: B32Register::new(),
            mode: B16Register::new(),
            ctrl: Ctrl::new(),
            baud_reload: B16Register::new(),
            padmc1040: Padmc1040::new(),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
    state.padmc.padmc1040.tx_fifo = NonNull::new(&mut state.padmc.tx_fifo as *mut Fifo<u8>);
    state.padmc.padmc1040.rx_fifo = NonNull::new(&mut state.padmc.rx_fifo as *mut Fifo<u8>);

    state.r3000.memory_mapper.map(0x1F80_1040, 4, &mut state.padmc.padmc1040 as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1044, 4, &mut state.padmc.stat as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_1048, 2, &mut state.padmc.mode as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_104A, 2, &mut state.padmc.ctrl as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x1F80_104E, 2, &mut state.padmc.baud_reload as *mut dyn B8MemoryMap);
}
