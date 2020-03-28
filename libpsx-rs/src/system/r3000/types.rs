use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::{B8MemoryMapper, B8MemoryMap};
use crate::types::mips1::branch_delay_slot::BranchDelaySlot;
use crate::system::r3000::constants::*;
use crate::system::types::State as SystemState;
use crate::system::r3000::cp0::types::State as Cp0State;
use crate::system::r3000::cp0::types::initialize as cp0_initialize;
use crate::system::r3000::cp2::types::State as Cp2State;
use crate::system::r3000::cp2::types::initialize as cp2_initialize;

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
