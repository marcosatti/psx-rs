use crate::types::{
    fifo::Fifo,
    memory::*,
    exclusive_state::ExclusiveState,
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    pub(crate) clock: f64,
    pub(crate) tx_enabled: bool,
    pub(crate) joy_select_enabled: bool,
    pub(crate) ack_interrupt_enabled: bool,
    pub(crate) use_joy2: bool,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
            tx_enabled: false,
            joy_select_enabled: false,
            ack_interrupt_enabled: false,
            use_joy2: false,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) rx_fifo: Fifo<u8>,
    pub(crate) tx_fifo: Fifo<u8>,
    pub(crate) stat: B32LevelRegister,
    pub(crate) mode: B16LevelRegister,
    pub(crate) ctrl: B16EdgeRegister,
    pub(crate) baud_reload: B16LevelRegister,
    pub(crate) controller_state: ExclusiveState<ControllerState>,
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
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
