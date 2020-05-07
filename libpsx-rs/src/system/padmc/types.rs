use crate::types::{
    fifo::{
        debug::DebugState,
        Fifo,
    },
    memory::*,
};
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

pub struct Ctrl {
    pub register: B16LevelRegister,
    pub write_latch: AtomicBool,
}

impl Ctrl {
    pub fn new() -> Ctrl {
        Ctrl {
            register: B16LevelRegister::new(),
            write_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u16(&self) -> u16 {
        self.register.read_u16()
    }

    pub fn write_u16(&self, value: u16) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u16(value);
    }
}

pub struct State {
    pub rx_fifo: Fifo<u8>,
    pub tx_fifo: Fifo<u8>,
    pub stat: B32LevelRegister,
    pub mode: B16LevelRegister,
    pub ctrl: Ctrl,
    pub baud_reload: B16LevelRegister,
}

impl State {
    pub fn new() -> State {
        State {
            rx_fifo: Fifo::new(16, Some(DebugState::new("PADMC RX", true, true))),
            tx_fifo: Fifo::new(16, Some(DebugState::new("PADMC TX", true, true))),
            stat: B32LevelRegister::new(),
            mode: B16LevelRegister::new(),
            ctrl: Ctrl::new(),
            baud_reload: B16LevelRegister::new(),
        }
    }
}
