use crate::types::{
    fifo::{
        debug::DebugState,
        Fifo,
    },
    memory::*,
};
use parking_lot::Mutex;

pub struct ControllerState {
    pub tx_enabled: bool,
    pub joy_select_enabled: bool,
    pub ack_interrupt_enabled: bool,
    pub use_joy2: bool,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            tx_enabled: false,
            joy_select_enabled: false,
            ack_interrupt_enabled: false,
            use_joy2: false,
        }
    }
}

pub struct State {
    pub rx_fifo: Fifo<u8>,
    pub tx_fifo: Fifo<u8>,
    pub stat: B32EdgeRegister,
    pub mode: B16LevelRegister,
    pub ctrl: B16EdgeRegister,
    pub baud_reload: B16LevelRegister,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            rx_fifo: Fifo::new(16, Some(DebugState::new("PADMC RX", true, true))),
            tx_fifo: Fifo::new(16, Some(DebugState::new("PADMC TX", true, true))),
            stat: B32EdgeRegister::new(),
            mode: B16LevelRegister::new(),
            ctrl: B16EdgeRegister::new(),
            baud_reload: B16LevelRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
