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
    types::{
        exclusive_state::ExclusiveState,
        mips1::{
            branch_delay_slot::BranchDelaySlot,
            instruction::Instruction,
            register::*,
        },
    },
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Hazard {
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

pub(crate) struct ControllerContext<'a> {
    pub(crate) state: &'a SystemState,
    pub(crate) r3000_state: &'a mut ControllerState,
    pub(crate) cp0_state: &'a mut Cp0ControllerState,
    pub(crate) cp2_state: &'a mut Cp2ControllerState,
}

pub(crate) type InstResult = Result<(), Hazard>;

pub(crate) type InstructionFn = fn(&mut ControllerContext, Instruction) -> InstResult;

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub(crate) struct ControllerState {
    pub(crate) clock: f64,
    pub(crate) pc: Register,
    pub(crate) branch_delay: BranchDelaySlot,
    pub(crate) gpr: [Register; 32],
    pub(crate) hi: Register,
    pub(crate) lo: Register,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
            pc: Register::new(),
            branch_delay: BranchDelaySlot::new(),
            gpr: [Register::new(); 32],
            hi: Register::new(),
            lo: Register::new(),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) cp0: Cp0State,
    pub(crate) cp2: Cp2State,
    pub(crate) controller_state: ExclusiveState<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            cp0: Cp0State::new(),
            cp2: Cp2State::new(),
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}

pub(crate) fn initialize(state: &mut SystemState) {
    state.r3000.controller_state.get_mut().pc.write_u32(0xBFC0_0000);
    cp0_initialize(state);
}
