pub mod cp0;
pub mod cp2;

use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::{B8MemoryMapper, B8MemoryMap};
use crate::types::mips1::branch_delay_slot::BranchDelaySlot;
use crate::constants::{BIOS_SIZE, MAIN_MEMORY_SIZE};
use crate::resources::Resources;
use crate::resources::r3000::cp0::Cp0;
use crate::resources::r3000::cp0::initialize as cp0_initialize;
use crate::resources::r3000::cp2::Cp2;
use crate::resources::r3000::cp2::initialize as cp2_initialize;

pub struct R3000 {
    pub pc: B32Register,
    pub branch_delay: BranchDelaySlot,
    pub gpr: [B32Register; 32],
    pub hi: B32Register,
    pub lo: B32Register,
    pub memory_mapper: B8MemoryMapper<u32>,
    pub cp0: Cp0,
    pub cp2: Cp2,
}

impl R3000 {
    pub fn new() -> R3000 {
        R3000 {
            pc: B32Register::new(),
            branch_delay: BranchDelaySlot::new(),
            gpr: [
                B32Register::read_only(0), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
                B32Register::new(), B32Register::new(), B32Register::new(), B32Register::new(), 
            ],
            hi: B32Register::new(),
            lo: B32Register::new(),
            memory_mapper: B8MemoryMapper::new(16, 16),
            cp0: Cp0::new(),
            cp2: Cp2::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1FC0_0000, BIOS_SIZE, &mut resources.bios as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x0000_0000, MAIN_MEMORY_SIZE, &mut resources.main_memory as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x0020_0000, MAIN_MEMORY_SIZE, &mut resources.main_memory as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x0040_0000, MAIN_MEMORY_SIZE, &mut resources.main_memory as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x0060_0000, MAIN_MEMORY_SIZE, &mut resources.main_memory as *mut dyn B8MemoryMap);
    
    resources.r3000.pc.write_u32(0xBFC0_0000);

    cp0_initialize(resources);

    cp2_initialize(resources);
}
