pub mod r3000;
pub mod memory_control;
pub mod intc;
pub mod timers;
pub mod spu;
pub mod dmac;
pub mod gpu;
pub mod cdrom;

use std::pin::Pin;
use std::marker::PhantomPinned;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use log::info;
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
use crate::resources::cdrom::Cdrom;
use crate::resources::cdrom::initialize as cdrom_initialize;

pub struct Resources {
    _pin: PhantomPinned,

    /// Bus lock status
    /// Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening. 
    /// The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing 
    /// the DMA controller to read/write garbage if the CPU is allowed to continue to run.
    pub bus_locked: bool, 

    pub r3000: R3000,
    pub intc: Intc,
    pub dmac: Dmac,
    pub timers: Timers,
    pub spu: Spu,
    pub memory_control: MemoryControl,
    pub gpu: Gpu,
    pub cdrom: Cdrom,
    pub bios: B8Memory,
    pub main_memory: B8Memory,
    pub post_display: B8Register,
    pub pio: B8Memory,
}

impl Resources {
    pub fn new() -> Pin<Box<Resources>> {
        let mut resources = Box::pin(Resources {
            _pin: PhantomPinned,
            bus_locked: false,
            r3000: R3000::new(),
            intc: Intc::new(),
            dmac: Dmac::new(),
            timers: Timers::new(),
            spu: Spu::new(),
            memory_control: MemoryControl::new(),
            gpu: Gpu::new(),
            cdrom: Cdrom::new(),
            bios: B8Memory::new(BIOS_SIZE),
            main_memory: B8Memory::new(MAIN_MEMORY_SIZE),
            post_display: B8Register::new(),
            pio: B8Memory::new_initialized(0x100, 0xFF),
        });

        Self::initialize(&mut resources);

        resources
    }

    fn initialize(resources: &mut Pin<Box<Resources>>) {
        let resources = unsafe { resources.as_mut().get_unchecked_mut() };

        r3000_initialize(resources);
        intc_initialize(resources);
        timers_initialize(resources);
        memory_control_initialize(resources);
        spu_initialize(resources);
        dmac_initialize(resources);
        gpu_initialize(resources);
        cdrom_initialize(resources);

        resources.r3000.memory_mapper.map::<u32>(0x1F80_2041, 1, &mut resources.post_display as *mut dyn B8MemoryMap);
        resources.r3000.memory_mapper.map::<u32>(0x1F00_0000, 0x100, &mut resources.pio as *mut dyn B8MemoryMap);
    }

    pub fn load_bios(resources: &mut Pin<Box<Resources>>, path: &PathBuf) {
        let resources = unsafe { resources.as_mut().get_unchecked_mut() };

        info!("Loading BIOS from {}", path.to_str().unwrap());
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        resources.bios.write_raw(&buffer, 0);
    }
}
