use crate::{
    system::{
        r3000::{
            cp0::types::{
                initialize as cp0_initialize,
                ControllerState as Cp0ControllerState,
                State as Cp0State,
            },
            cp2::types::{
                ControllerState as Cp2ControllerState,
                State as Cp2State,
            },
        },
        types::State as SystemState,
    },
    types::mips1::{
        branch_delay_slot::BranchDelaySlot,
        instruction::Instruction,
        register::*,
    },
};
use parking_lot::Mutex;
use std::fmt;

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

pub struct ControllerContext<'a> {
    pub state: &'a SystemState,
    pub r3000_state: &'a mut ControllerState,
    pub cp0_state: &'a mut Cp0ControllerState,
    pub cp2_state: &'a mut Cp2ControllerState,
}

pub type InstResult = Result<(), Hazard>;

pub type InstructionFn = fn(&mut ControllerContext, Instruction) -> InstResult;

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
