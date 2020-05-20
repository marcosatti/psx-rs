use crate::types::{
    fifo::Fifo,
    memory::*,
};
use parking_lot::Mutex;
use std::collections::VecDeque;

pub(crate) struct ControllerState {
    pub(crate) interrupt_index: usize,
    /// Command state.
    pub(crate) command_index: Option<u8>,
    pub(crate) command_iteration: usize,
    /// Seeking status.
    pub(crate) seeking: bool,
    /// Reading status.
    pub(crate) reading: bool,
    /// Current MSF address.
    /// Base is stored in BCD format.
    pub(crate) msf_address_base: (u8, u8, u8),
    pub(crate) msf_address_offset: usize,
    pub(crate) sector_delay_counter: usize,
    pub(crate) sector_buffer: VecDeque<u8>,
    pub(crate) load_data_flag: bool,
    pub(crate) loading_data: bool,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            interrupt_index: 0,
            command_index: None,
            command_iteration: 0,
            seeking: false,
            reading: false,
            msf_address_base: (0, 0, 0),
            msf_address_offset: 0,
            sector_delay_counter: 0,
            sector_buffer: VecDeque::new(),
            load_data_flag: false,
            loading_data: false,
        }
    }
}

pub(crate) struct State {
    pub(crate) status: B8LevelRegister,
    pub(crate) response: Fifo<u8>,
    pub(crate) parameter: Fifo<u8>,
    pub(crate) data: Fifo<u8>,
    pub(crate) command: B8EdgeRegister,
    pub(crate) interrupt_enable: B8LevelRegister,
    pub(crate) interrupt_flag: B8EdgeRegister,
    pub(crate) request: B8EdgeRegister,
    pub(crate) controller_state: Mutex<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            status: B8LevelRegister::new(),
            response: Fifo::new(16),
            parameter: Fifo::new(16),
            data: Fifo::new(2048),
            command: B8EdgeRegister::new(),
            interrupt_enable: B8LevelRegister::new(),
            interrupt_flag: B8EdgeRegister::new(),
            request: B8EdgeRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
