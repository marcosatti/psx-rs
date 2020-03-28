use std::pin::Pin;
use std::marker::PhantomPinned;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use log::info;
use crate::system::r3000::constants::{BIOS_SIZE, MAIN_MEMORY_SIZE};
use crate::types::memory::b8_memory::B8Memory;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::system::r3000::R3000;
use crate::system::r3000::initialize as r3000_initialize;
use crate::system::intc::Intc;
use crate::system::intc::initialize as intc_initialize;
use crate::system::timers::Timers;
use crate::system::timers::initialize as timers_initialize;
use crate::system::spu::Spu;
use crate::system::spu::initialize as spu_initialize;
use crate::system::memory_control::MemoryControl;
use crate::system::memory_control::initialize as memory_control_initialize;
use crate::system::dmac::Dmac;
use crate::system::dmac::initialize as dmac_initialize;
use crate::system::gpu::Gpu;
use crate::system::gpu::initialize as gpu_initialize;
use crate::system::cdrom::Cdrom;
use crate::system::cdrom::initialize as cdrom_initialize;
use crate::system::padmc::Padmc;
use crate::system::padmc::initialize as padmc_initialize;

pub struct State {
    _pin: PhantomPinned,

    /// Bus lock status
    /// Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening. 
    /// The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing 
    /// the DMA controller to read/write garbage if the CPU is allowed to continue to run.
    pub bus_locked: AtomicBool, 

    pub r3000: R3000,
    pub intc: Intc,
    pub dmac: Dmac,
    pub timers: Timers,
    pub spu: Spu,
    pub memory_control: MemoryControl,
    pub gpu: Gpu,
    pub cdrom: Cdrom,
    pub padmc: Padmc,
    pub bios: B8Memory,
    pub main_memory: B8Memory,
    pub post_display: B8Register,
    pub pio: B8Memory,
}

impl State {
    pub fn new() -> Pin<Box<State>> {
        Box::pin(State {
            _pin: PhantomPinned,
            bus_locked: AtomicBool::new(false),
            r3000: R3000::new(),
            intc: Intc::new(),
            dmac: Dmac::new(),
            timers: Timers::new(),
            spu: Spu::new(),
            memory_control: MemoryControl::new(),
            gpu: Gpu::new(),
            cdrom: Cdrom::new(),
            padmc: Padmc::new(),
            bios: B8Memory::new(BIOS_SIZE),
            main_memory: B8Memory::new(MAIN_MEMORY_SIZE),
            post_display: B8Register::new(),
            pio: B8Memory::new_initialized(0x100, 0xFF),
        })
    }

    pub fn initialize(state: &mut State) {
        r3000_initialize(state);
        intc_initialize(state);
        timers_initialize(state);
        memory_control_initialize(state);
        spu_initialize(state);
        dmac_initialize(state);
        gpu_initialize(state);
        cdrom_initialize(state);
        padmc_initialize(state);

        state.r3000.memory_mapper.map(0x1F80_2041, 1, &mut state.post_display as *mut dyn B8MemoryMap);
        state.r3000.memory_mapper.map(0x1F00_0000, 0x100, &mut state.pio as *mut dyn B8MemoryMap);
    }

    pub fn load_bios(state: &mut State, path: &Path) {
        info!("Loading BIOS from {}", path.to_str().unwrap());
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        state.bios.write_raw(&buffer, 0);
    }
}
