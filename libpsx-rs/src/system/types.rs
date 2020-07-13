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
};
use log::info;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum Event {
    Time(f64),
}

pub(crate) struct ControllerContext<'a: 'b, 'b: 'c, 'c> {
    pub(crate) state: &'c State,
    pub(crate) video_backend: &'c VideoBackend<'a, 'b>,
    pub(crate) audio_backend: &'c AudioBackend<'a, 'b>,
    pub(crate) cdrom_backend: &'c CdromBackend<'a, 'b>,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
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
    pub(crate) bus_locked: AtomicBool,
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
            bus_locked: AtomicBool::new(false),
        })
    }

    pub(crate) fn initialize(state: &mut State) {
        r3000_initialize(state);
    }

    pub(crate) fn load_bios(state: &mut State, path: &Path) {
        info!("Loading BIOS from {}", path.to_str().unwrap());
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        state.memory.bios.write_raw(0, &buffer);
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            r3000: self.r3000.clone(),
            intc: self.intc.clone(),
            dmac: self.dmac.clone(),
            timers: self.timers.clone(),
            spu: self.spu.clone(),
            memory: self.memory.clone(),
            gpu: self.gpu.clone(),
            cdrom: self.cdrom.clone(),
            padmc: self.padmc.clone(),
            bus_locked: AtomicBool::new(self.bus_locked.load(Ordering::Relaxed)),
        }
    }
}
