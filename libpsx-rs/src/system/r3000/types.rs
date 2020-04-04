use crate::{
    system::{
        r3000::{
            constants::*,
            cp0::types::{
                initialize as cp0_initialize,
                State as Cp0State,
            },
            cp2::types::{
                initialize as cp2_initialize,
                State as Cp2State,
            },
        },
        types::State as SystemState,
    },
    types::{
        b8_memory_mapper::{
            B8MemoryMap,
            B8MemoryMapper,
        },
        mips1::branch_delay_slot::BranchDelaySlot,
        register::b32_register::B32Register,
    },
};
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

pub type InstResult = Result<(), Hazard>;

pub struct State {
    pub pc: B32Register,
    pub branch_delay: BranchDelaySlot,
    pub gpr: [B32Register; 32],
    pub hi: B32Register,
    pub lo: B32Register,
    pub memory_mapper: B8MemoryMapper<u32>,
    pub cp0: Cp0State,
    pub cp2: Cp2State,
}

impl State {
    pub fn new() -> State {
        State {
            pc: B32Register::new(),
            branch_delay: BranchDelaySlot::new(),
            gpr: [B32Register::new(); 32],
            hi: B32Register::new(),
            lo: B32Register::new(),
            memory_mapper: B8MemoryMapper::new(16, 16),
            cp0: Cp0State::new(),
            cp2: Cp2State::new(),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
    state.r3000.memory_mapper.map(0x1FC0_0000, BIOS_SIZE, &mut state.bios as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x0000_0000, MAIN_MEMORY_SIZE, &mut state.main_memory as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x0020_0000, MAIN_MEMORY_SIZE, &mut state.main_memory as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x0040_0000, MAIN_MEMORY_SIZE, &mut state.main_memory as *mut dyn B8MemoryMap);
    state.r3000.memory_mapper.map(0x0060_0000, MAIN_MEMORY_SIZE, &mut state.main_memory as *mut dyn B8MemoryMap);

    state.r3000.pc.write_u32(0xBFC0_0000);

    cp0_initialize(state);

    cp2_initialize(state);
}
