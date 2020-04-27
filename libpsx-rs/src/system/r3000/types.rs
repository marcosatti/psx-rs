use crate::{
    system::{
        r3000::{
            cp0::types::{
                initialize as cp0_initialize,
                State as Cp0State,
            },
            cp2::types::State as Cp2State,
        },
        types::State as SystemState,
    },
    types::mips1::{
        register::*,
        branch_delay_slot::BranchDelaySlot,
    },
};
use std::fmt;
use parking_lot::Mutex;

#[derive(Copy, Clone, PartialEq)]
pub enum Hazard {
    BusLockedMemoryRead(u32),
    BusLockedMemoryWrite(u32),
    MemoryRead(u32),
    MemoryWrite(u32),
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hazard::BusLockedMemoryRead(a) => write!(f, "BusLockedMemoryRead(0x{:08X})", a),
            Hazard::BusLockedMemoryWrite(a) => write!(f, "BusLockedMemoryWrite(0x{:08X})", a),
            Hazard::MemoryRead(a) => write!(f, "MemoryRead(0x{:08X})", a),
            Hazard::MemoryWrite(a) => write!(f, "MemoryWrite(0x{:08X})", a),
        }
    }
}

impl fmt::Debug for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub type InstResult = Result<(), Hazard>;

pub struct ControllerState {
    pub pc: Register,
    pub branch_delay: BranchDelaySlot,
    pub gpr: [Register; 32],
    pub hi: Register,
    pub lo: Register,
}

impl ControllerState {
    pub fn new() -> ControllerState { 
        ControllerState {
            pc: Register::new(),
            branch_delay: BranchDelaySlot::new(),
            gpr: [Register::new(); 32],
            hi: Register::new(),
            lo: Register::new(),
        }
    }
}

pub struct State {
    pub cp0: Cp0State,
    pub cp2: Cp2State,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            cp0: Cp0State::new(),
            cp2: Cp2State::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
    state.r3000.controller_state.get_mut().pc.write_u32(0xBFC0_0000);
    cp0_initialize(state);
}
