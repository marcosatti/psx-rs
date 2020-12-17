use crate::{
    system::intc::constants::*,
    types::{
        exclusive_state::ExclusiveState,
        flag::Flag,
        memory::*,
    },
    utilities::bool_to_flag,
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Copy, Clone)]
pub(crate) enum Line {
    Vblank,
    Gpu,
    Cdrom,
    Dma,
    Tmr0,
    Tmr1,
    Tmr2,
    Padmc,
    Sio,
    Spu,
    Pio,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct Stat {
    vblank: Flag,
    gpu: Flag,
    cdrom: Flag,
    dma: Flag,
    tmr0: Flag,
    tmr1: Flag,
    tmr2: Flag,
    padmc: Flag,
    sio: Flag,
    spu: Flag,
    pio: Flag,
}

impl Stat {
    pub(crate) fn new() -> Stat {
        Stat {
            vblank: Flag::new(),
            gpu: Flag::new(),
            cdrom: Flag::new(),
            dma: Flag::new(),
            tmr0: Flag::new(),
            tmr1: Flag::new(),
            tmr2: Flag::new(),
            padmc: Flag::new(),
            sio: Flag::new(),
            spu: Flag::new(),
            pio: Flag::new(),
        }
    }

    pub(crate) fn assert_line(&self, line: Line) {
        match line {
            Line::Vblank => self.vblank.store(true),
            Line::Gpu => self.gpu.store(true),
            Line::Cdrom => self.cdrom.store(true),
            Line::Dma => self.dma.store(true),
            Line::Tmr0 => self.tmr0.store(true),
            Line::Tmr1 => self.tmr1.store(true),
            Line::Tmr2 => self.tmr2.store(true),
            Line::Padmc => self.padmc.store(true),
            Line::Sio => self.sio.store(true),
            Line::Spu => self.spu.store(true),
            Line::Pio => self.pio.store(true),
        }
    }

    fn acknowledge(&self, acknowledge_mask: u32) {
        for i in 0..32 {
            let acknowledged = ((acknowledge_mask >> i) & 1) == 0;
            if acknowledged {
                match i {
                    0 => self.vblank.store(false),
                    1 => self.gpu.store(false),
                    2 => self.cdrom.store(false),
                    3 => self.dma.store(false),
                    4 => self.tmr0.store(false),
                    5 => self.tmr1.store(false),
                    6 => self.tmr2.store(false),
                    7 => self.padmc.store(false),
                    8 => self.sio.store(false),
                    9 => self.spu.store(false),
                    10 => self.pio.store(false),
                    // Ignore (always zero).
                    _ => {},
                }
            }
        }
    }

    pub(crate) fn value(&self) -> u32 {
        let mut value = 0;
        value = VBLANK.insert_into(value, bool_to_flag(self.vblank.load()));
        value = GPU.insert_into(value, bool_to_flag(self.gpu.load()));
        value = CDROM.insert_into(value, bool_to_flag(self.cdrom.load()));
        value = DMA.insert_into(value, bool_to_flag(self.dma.load()));
        value = TMR0.insert_into(value, bool_to_flag(self.tmr0.load()));
        value = TMR1.insert_into(value, bool_to_flag(self.tmr1.load()));
        value = TMR2.insert_into(value, bool_to_flag(self.tmr2.load()));
        value = PADMC.insert_into(value, bool_to_flag(self.padmc.load()));
        value = SIO.insert_into(value, bool_to_flag(self.sio.load()));
        value = SPU.insert_into(value, bool_to_flag(self.spu.load()));
        value = PIO.insert_into(value, bool_to_flag(self.pio.load()));
        value
    }

    pub(crate) fn read_u16(&self, offset: u32) -> u16 {
        assert_eq!(offset, 0);
        self.value() as u16
    }

    pub(crate) fn write_u16(&self, offset: u32, value: u16) {
        assert_eq!(offset, 0);
        self.acknowledge(value as u32)
    }

    pub(crate) fn read_u32(&self) -> u32 {
        self.value() as u32
    }

    pub(crate) fn write_u32(&self, value: u32) {
        self.acknowledge(value)
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    pub(crate) clock: f32,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) controller_state: ExclusiveState<ControllerState>,
    pub(crate) stat: Stat,
    pub(crate) mask: B32LevelRegister,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            controller_state: ExclusiveState::new(ControllerState::new()),
            stat: Stat::new(),
            mask: B32LevelRegister::new(),
        }
    }
}
