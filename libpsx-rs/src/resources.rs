pub mod r3000;
pub mod memory_control;
pub mod intc;
pub mod timers;
pub mod spu;
pub mod dmac;
pub mod gpu;

use crate::constants::{BIOS_SIZE, MAIN_MEMORY_SIZE};
use crate::types::memory::b8_memory::B8Memory;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::resources::r3000::R3000;
use crate::resources::r3000::initialize as r3000_initialize;
use crate::resources::intc::Intc;
use crate::resources::intc::initialize as intc_initialize;
use crate::resources::timers::Timers;
use crate::resources::timers::initialize as timers_initialize;
use crate::resources::spu::Spu;
use crate::resources::spu::initialize as spu_initialize;
use crate::resources::memory_control::MemoryControl;
use crate::resources::memory_control::initialize as memory_control_initialize;
use crate::resources::dmac::Dmac;
use crate::resources::dmac::initialize as dmac_initialize;
use crate::resources::gpu::Gpu;
use crate::resources::gpu::initialize as gpu_initialize;

pub struct Resources {
    pub bus_locked: bool, // Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening. The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing the DMA controller to read/write garbage if the CPU is allowed to continue to run.

    pub r3000: R3000,
    pub intc: Intc,
    pub dmac: Dmac,
    pub timers: Timers,
    pub spu: Spu,
    pub memory_control: MemoryControl,
    pub gpu: Gpu,
    pub bios: B8Memory,
    pub main_memory: B8Memory,
    pub post_display: B8Register,
    pub pio: B8Memory,
}

impl Resources {
    pub fn new() -> Box<Resources> {
        let mut resources = box Resources {
            bus_locked: false,
            r3000: R3000::new(),
            intc: Intc::new(),
            dmac: Dmac::new(),
            timers: Timers::new(),
            spu: Spu::new(),
            memory_control: MemoryControl::new(),
            gpu: Gpu::new(),
            bios: B8Memory::new(BIOS_SIZE),
            main_memory: B8Memory::new(MAIN_MEMORY_SIZE),
            post_display: B8Register::new(),
            pio: B8Memory::new_initialized(0x100, 0xFF),
        };

        initialize(&mut resources);
        resources
    }
}

fn initialize(resources: &mut Resources) {
    r3000_initialize(resources);
    intc_initialize(resources);
    timers_initialize(resources);
    memory_control_initialize(resources);
    spu_initialize(resources);
    dmac_initialize(resources);
    gpu_initialize(resources);

    resources.r3000.memory_mapper.map::<u32>(0x1F80_2041, 1, &mut resources.post_display as *mut B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F00_0000, 0x100, &mut resources.pio as *mut B8MemoryMap);
}
