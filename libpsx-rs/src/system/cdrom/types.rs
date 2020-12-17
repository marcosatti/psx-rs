use crate::{
    backends::cdrom::CdromBackend,
    system::types::{
        ControllerResult,
        State as SystemState,
    },
    types::{
        exclusive_state::ExclusiveState,
        fifo::Fifo,
        memory::*,
    },
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::VecDeque;

pub(crate) type WaitCyclesFn = fn(usize) -> ControllerResult<usize>;

pub(crate) type LengthFn = fn(usize) -> usize;

pub(crate) type HandlerFn = fn(&SystemState, &mut ControllerState, &CdromBackend, usize) -> ControllerResult<bool>;

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub(crate) enum WaitCyclesMode {
    Ready,
    Waiting(usize),
    Executing,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    // Synchronization state.
    pub(crate) clock: f32,
    // Interrupt state.
    pub(crate) interrupt_index: usize,
    /// Command state.
    pub(crate) command_index: Option<u8>,
    pub(crate) command_iteration: usize,
    pub(crate) command_wait_cycles: WaitCyclesMode,
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
            clock: 0.0,
            interrupt_index: 0,
            command_index: None,
            command_iteration: 0,
            command_wait_cycles: WaitCyclesMode::Ready,
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

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) status: B8LevelRegister,
    pub(crate) response: Fifo<u8>,
    pub(crate) parameter: Fifo<u8>,
    pub(crate) data: Fifo<u8>,
    pub(crate) command: B8EdgeRegister,
    pub(crate) interrupt_enable: B8LevelRegister,
    pub(crate) interrupt_flag: B8EdgeRegister,
    pub(crate) request: B8EdgeRegister,
    pub(crate) audio_left_to_left: B8LevelRegister,
    pub(crate) audio_left_to_right: B8LevelRegister,
    pub(crate) audio_right_to_left: B8LevelRegister,
    pub(crate) audio_right_to_right: B8LevelRegister,
    pub(crate) audio_apply: B8EdgeRegister,
    pub(crate) controller_state: ExclusiveState<ControllerState>,
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
            interrupt_flag: B8EdgeRegister::with_value(0xE0),
            request: B8EdgeRegister::new(),
            audio_left_to_left: B8LevelRegister::new(),
            audio_left_to_right: B8LevelRegister::new(),
            audio_right_to_left: B8LevelRegister::new(),
            audio_right_to_right: B8LevelRegister::new(),
            audio_apply: B8EdgeRegister::new(),
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
