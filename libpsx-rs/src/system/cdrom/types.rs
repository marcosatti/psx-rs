use crate::types::{
    fifo::{
        debug::DebugState,
        Fifo,
    },
    memory::*,
};
use parking_lot::Mutex;
use std::collections::VecDeque;

pub struct ControllerState {
    pub interrupt_index: usize,
    /// Command state.
    pub command_index: Option<u8>,
    pub command_iteration: usize,
    /// Seeking status.
    pub seeking: bool,
    /// Reading status.
    pub reading: bool,
    /// Current MSF address.
    /// Base is stored in BCD format.
    pub msf_address_base: (u8, u8, u8),
    pub msf_address_offset: usize,
    pub sector_delay_counter: usize,
    pub sector_buffer: VecDeque<u8>,
    pub load_data_flag: bool,
    pub loading_data: bool,
}

impl ControllerState {
    pub fn new() -> ControllerState {
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

pub struct State {
    pub status: B8LevelRegister,
    pub response: Fifo<u8>,
    pub parameter: Fifo<u8>,
    pub data: Fifo<u8>,
    pub command: B8EdgeRegister,
    pub interrupt_enable: B8LevelRegister,
    pub interrupt_flag: B8EdgeRegister,
    pub request: B8EdgeRegister,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            status: B8LevelRegister::new(),
            response: Fifo::new(16, Some(DebugState::new("CDROM RESPONSE", true, true))),
            parameter: Fifo::new(16, Some(DebugState::new("CDROM PARAMETER", true, true))),
            data: Fifo::new(64, Some(DebugState::new("CDROM DATA", true, true))),
            command: B8EdgeRegister::new(),
            interrupt_enable: B8LevelRegister::new(),
            interrupt_flag: B8EdgeRegister::new(),
            request: B8EdgeRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
