use crate::types::mips1::{
    instruction::Instruction,
    register::*,
};
use parking_lot::Mutex;

pub enum MultiplyMatrix {
    Rotation,
    Light,
    Color,
    Reserved,
}

pub enum MultiplyVector {
    V0,
    V1,
    V2,
    IR,
}

pub enum TranslationVector {
    TR,
    BK,
    FC,
    None,
}

pub struct GteInstruction {
    pub instruction: Instruction,
}

impl GteInstruction {
    pub fn new(instruction: Instruction) -> GteInstruction {
        GteInstruction {
            instruction,
        }
    }

    pub fn fake(&self) -> usize {
        ((self.instruction.value >> 20) & 0x1F) as usize
    }

    pub fn sf(&self) -> bool {
        ((self.instruction.value >> 19) & 0x1) > 0
    }

    pub fn mvmva_mm(&self) -> MultiplyMatrix {
        match (self.instruction.value >> 17) & 0x3 {
            0 => MultiplyMatrix::Rotation,
            1 => MultiplyMatrix::Light,
            2 => MultiplyMatrix::Color,
            3 => MultiplyMatrix::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn mvmva_mv(&self) -> MultiplyVector {
        match (self.instruction.value >> 15) & 0x3 {
            0 => MultiplyVector::V0,
            1 => MultiplyVector::V1,
            2 => MultiplyVector::V2,
            3 => MultiplyVector::IR,
            _ => unreachable!(),
        }
    }

    pub fn mvmva_tv(&self) -> TranslationVector {
        match (self.instruction.value >> 13) & 0x3 {
            0 => TranslationVector::TR,
            1 => TranslationVector::BK,
            2 => TranslationVector::FC,
            3 => TranslationVector::None,
            _ => unreachable!(),
        }
    }

    pub fn lm(&self) -> bool {
        ((self.instruction.value >> 10) & 0x1) > 0
    }

    pub fn cmd(&self) -> usize {
        (self.instruction.value & 0x1F) as usize
    }
}

pub struct ControllerState {
    /// Data registers.
    pub gd: [Register; 32],
    /// Control registers.
    pub gc: [Register; 32],
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            gd: [Register::new(); 32],
            gc: [Register::new(); 32],
        }
    }
}

pub struct State {
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
