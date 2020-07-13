use crate::types::mips1::{
    instruction::Instruction,
    register::*,
};
use parking_lot::Mutex;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[allow(dead_code)]
pub(crate) enum MultiplyMatrix {
    Rotation,
    Light,
    Color,
    Reserved,
}

#[allow(dead_code)]
pub(crate) enum MultiplyVector {
    V0,
    V1,
    V2,
    IR,
}

#[allow(dead_code)]
pub(crate) enum TranslationVector {
    TR,
    BK,
    FC,
    None,
}

pub(crate) struct GteInstruction {
    pub(crate) instruction: Instruction,
}

impl GteInstruction {
    pub(crate) fn new(instruction: Instruction) -> GteInstruction {
        GteInstruction {
            instruction,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn fake(&self) -> usize {
        ((self.instruction.value >> 20) & 0x1F) as usize
    }

    pub(crate) fn sf(&self) -> bool {
        ((self.instruction.value >> 19) & 0x1) > 0
    }

    #[allow(dead_code)]
    pub(crate) fn mvmva_mm(&self) -> MultiplyMatrix {
        match (self.instruction.value >> 17) & 0x3 {
            0 => MultiplyMatrix::Rotation,
            1 => MultiplyMatrix::Light,
            2 => MultiplyMatrix::Color,
            3 => MultiplyMatrix::Reserved,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn mvmva_mv(&self) -> MultiplyVector {
        match (self.instruction.value >> 15) & 0x3 {
            0 => MultiplyVector::V0,
            1 => MultiplyVector::V1,
            2 => MultiplyVector::V2,
            3 => MultiplyVector::IR,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn mvmva_tv(&self) -> TranslationVector {
        match (self.instruction.value >> 13) & 0x3 {
            0 => TranslationVector::TR,
            1 => TranslationVector::BK,
            2 => TranslationVector::FC,
            3 => TranslationVector::None,
            _ => unreachable!(),
        }
    }

    pub(crate) fn lm(&self) -> bool {
        ((self.instruction.value >> 10) & 0x1) > 0
    }

    #[allow(dead_code)]
    pub(crate) fn cmd(&self) -> usize {
        (self.instruction.value & 0x1F) as usize
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    /// Data registers.
    pub(crate) gd: [Register; 32],
    /// Control registers.
    pub(crate) gc: [Register; 32],
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            gd: [Register::new(); 32],
            gc: [Register::new(); 32],
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct State {
    pub(crate) controller_state: Mutex<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            controller_state: Mutex::new(self.controller_state.lock().clone()),
        }
    }
}
