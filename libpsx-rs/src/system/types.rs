use crate::{
    backends::{
        audio::AudioBackend,
        cdrom::CdromBackend,
        video::VideoBackend,
    },
    system::{
        cdrom::types::{
            initialize as cdrom_initialize,
            State as CdromState,
        },
        dmac::types::{
            initialize as dmac_initialize,
            State as DmacState,
        },
        gpu::types::{
            initialize as gpu_initialize,
            State as GpuState,
        },
        intc::types::{
            initialize as intc_initialize,
            State as IntcState,
        },
        memory_control::types::{
            initialize as memory_control_initialize,
            State as MemoryControlState,
        },
        padmc::types::{
            initialize as padmc_initialize,
            State as PadmcState,
        },
        r3000::{
            constants::{
                BIOS_SIZE,
                MAIN_MEMORY_SIZE,
            },
            types::{
                initialize as r3000_initialize,
                State as R3000State,
            },
        },
        spu::types::{
            initialize as spu_initialize,
            State as SpuState,
        },
        timers::types::{
            initialize as timers_initialize,
            State as TimersState,
        },
    },
    types::{
        b8_memory_mapper::B8MemoryMap,
        memory::b8_memory::B8Memory,
        register::b8_register::B8Register,
    },
    Context,
};
use log::info;
use std::{
    fs::File,
    io::Read,
    marker::PhantomPinned,
    path::Path,
    pin::Pin,
    sync::atomic::AtomicBool,
    time::Duration,
};

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Time(Duration),
}

pub struct ControllerContext<'a: 'b, 'b: 'c, 'c> {
    pub state: &'c mut State,
    pub video_backend: &'c VideoBackend<'a, 'b>,
    pub audio_backend: &'c AudioBackend<'a, 'b>,
    pub cdrom_backend: &'c CdromBackend<'a, 'b>,
}

impl<'a: 'b, 'b: 'c, 'c> ControllerContext<'a, 'b, 'c> {
    pub unsafe fn from_core_context(context: &Context<'a, 'b, 'c>) -> ControllerContext<'a, 'b, 'c> {
        ControllerContext {
            state: context.state.as_mut().unwrap(),
            video_backend: context.video_backend,
            audio_backend: context.audio_backend,
            cdrom_backend: context.cdrom_backend,
        }
    }
}

pub struct State {
    _pin: PhantomPinned,

    /// Bus lock status
    /// Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening.
    /// The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing
    /// the DMA controller to read/write garbage if the CPU is allowed to continue to run.
    pub bus_locked: AtomicBool,

    pub r3000: R3000State,
    pub intc: IntcState,
    pub dmac: DmacState,
    pub timers: TimersState,
    pub spu: SpuState,
    pub memory_control: MemoryControlState,
    pub gpu: GpuState,
    pub cdrom: CdromState,
    pub padmc: PadmcState,
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
            r3000: R3000State::new(),
            intc: IntcState::new(),
            dmac: DmacState::new(),
            timers: TimersState::new(),
            spu: SpuState::new(),
            memory_control: MemoryControlState::new(),
            gpu: GpuState::new(),
            cdrom: CdromState::new(),
            padmc: PadmcState::new(),
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
