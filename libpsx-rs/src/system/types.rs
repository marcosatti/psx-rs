use crate::{
    backends::{
        audio::AudioBackend,
        cdrom::CdromBackend,
        video::VideoBackend,
    },
    system::{
        cdrom::types::State as CdromState,
        dmac::types::State as DmacState,
        gpu::types::State as GpuState,
        intc::types::State as IntcState,
        memory::types::State as MemoryState,
        padmc::types::State as PadmcState,
        r3000::types::{
            initialize as r3000_initialize,
            State as R3000State,
        },
        spu::types::State as SpuState,
        timers::types::State as TimersState,
    },
    types::flag::Flag,
};
use log::info;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs::File,
    io::{
        Read,
        Result as IoResult,
    },
    path::Path,
};

pub(crate) type ControllerHandler = fn(&ControllerContext, Event) -> ControllerResult<()>;

pub(crate) type ControllerResult<T> = Result<T, String>;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Event {
    Time(f32),
}

pub(crate) struct ControllerContext<'a: 'b, 'b> {
    pub(crate) state: &'b State,
    pub(crate) video_backend: &'b VideoBackend<'a>,
    pub(crate) audio_backend: &'b AudioBackend<'a>,
    pub(crate) cdrom_backend: &'b CdromBackend<'a>,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) r3000: R3000State,
    pub(crate) intc: IntcState,
    pub(crate) dmac: DmacState,
    pub(crate) timers: TimersState,
    pub(crate) spu: SpuState,
    pub(crate) memory: MemoryState,
    pub(crate) gpu: GpuState,
    pub(crate) cdrom: CdromState,
    pub(crate) padmc: PadmcState,

    /// Bus lock status
    /// Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening.
    /// The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing
    /// the DMA controller to read/write garbage if the CPU is allowed to continue to run.
    pub(crate) bus_locked: Flag,
}

impl State {
    pub(crate) fn new() -> Box<State> {
        Box::new(State {
            r3000: R3000State::new(),
            intc: IntcState::new(),
            dmac: DmacState::new(),
            timers: TimersState::new(),
            spu: SpuState::new(),
            memory: MemoryState::new(),
            gpu: GpuState::new(),
            cdrom: CdromState::new(),
            padmc: PadmcState::new(),
            bus_locked: Flag::new(),
        })
    }

    pub(crate) fn initialize(state: &mut State) {
        r3000_initialize(state);
    }

    pub(crate) fn load_bios(state: &mut State, path: &Path) -> IoResult<()> {
        info!("Loading BIOS from {}", path.to_str().unwrap());
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        state.memory.bios.write_raw(0, &buffer);
        Ok(())
    }

    pub(crate) fn with_bios(prefix: &Path, name: &str) -> IoResult<Box<State>> {
        let mut state = State::new();
        State::initialize(&mut state);
        State::load_bios(&mut state, &prefix.join(r"bios/").join(name))?;
        Ok(state)
    }
}
