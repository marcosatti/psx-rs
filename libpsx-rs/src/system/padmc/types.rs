use crate::types::{
    fifo::Fifo,
    memory::*,
};
use parking_lot::Mutex;

pub(crate) struct ControllerState {
    pub(crate) tx_enabled: bool,
    pub(crate) joy_select_enabled: bool,
    pub(crate) ack_interrupt_enabled: bool,
    pub(crate) use_joy2: bool,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            tx_enabled: false,
            joy_select_enabled: false,
            ack_interrupt_enabled: false,
            use_joy2: false,
        }
    }
}

pub(crate) struct State {
    pub(crate) rx_fifo: Fifo<u8>,
    pub(crate) tx_fifo: Fifo<u8>,
    pub(crate) stat: B32LevelRegister,
    pub(crate) mode: B16LevelRegister,
    pub(crate) ctrl: B16EdgeRegister,
    pub(crate) baud_reload: B16LevelRegister,
    pub(crate) controller_state: Mutex<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            rx_fifo: Fifo::new(16),
            tx_fifo: Fifo::new(16),
            stat: B32LevelRegister::new(),
            mode: B16LevelRegister::new(),
            ctrl: B16EdgeRegister::new(),
            baud_reload: B16LevelRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
