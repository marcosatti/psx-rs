use crate::{
    system::intc::constants::*,
    types::{
        exclusive_state::ExclusiveState,
        memory::*,
    },
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
    register: B32LevelRegister,
}

impl Stat {
    pub(crate) fn new() -> Stat {
        Stat {
            register: B32LevelRegister::new(),
        }
    }

    pub(crate) fn assert_line(&self, line: Line) {
        match line {
            Line::Vblank => self.register.write_bitfield_atomic(VBLANK, 1),
            Line::Gpu => self.register.write_bitfield_atomic(GPU, 1),
            Line::Cdrom => self.register.write_bitfield_atomic(CDROM, 1),
            Line::Dma => self.register.write_bitfield_atomic(DMA, 1),
            Line::Tmr0 => self.register.write_bitfield_atomic(TMR0, 1),
            Line::Tmr1 => self.register.write_bitfield_atomic(TMR1, 1),
            Line::Tmr2 => self.register.write_bitfield_atomic(TMR2, 1),
            Line::Padmc => self.register.write_bitfield_atomic(PADMC, 1),
            Line::Sio => self.register.write_bitfield_atomic(SIO, 1),
            Line::Spu => self.register.write_bitfield_atomic(SPU, 1),
            Line::Pio => self.register.write_bitfield_atomic(PIO, 1),
        }
    }

    fn acknowledge(&self, acknowledge_mask: u32) {
        for i in 0..32 {
            let acknowledged = ((acknowledge_mask >> i) & 1) == 0;
            if acknowledged {
                match i {
                    0 => self.register.write_bitfield_atomic(VBLANK, 0),
                    1 => self.register.write_bitfield_atomic(GPU, 0),
                    2 => self.register.write_bitfield_atomic(CDROM, 0),
                    3 => self.register.write_bitfield_atomic(DMA, 0),
                    4 => self.register.write_bitfield_atomic(TMR0, 0),
                    5 => self.register.write_bitfield_atomic(TMR1, 0),
                    6 => self.register.write_bitfield_atomic(TMR2, 0),
                    7 => self.register.write_bitfield_atomic(PADMC, 0),
                    8 => self.register.write_bitfield_atomic(SIO, 0),
                    9 => self.register.write_bitfield_atomic(SPU, 0),
                    10 => self.register.write_bitfield_atomic(PIO, 0),
                    // Ignore (always zero).
                    _ => {},
                }
            }
        }
    }

    pub(crate) fn read_u16(&self, offset: u32) -> u16 {
        assert_eq!(offset, 0);
        self.register.read_u16(0)
    }

    pub(crate) fn write_u16(&self, offset: u32, value: u16) {
        assert_eq!(offset, 0);
        self.acknowledge(value as u32)
    }

    pub(crate) fn read_u32(&self) -> u32 {
        self.register.read_u32()
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
