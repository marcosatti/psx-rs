pub mod register;

use std::ptr::NonNull;
use crate::types::register::b32_register::B32Register;
use crate::types::register::b16_register::B16Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;
use crate::resources::Resources;
use crate::resources::padmc::register::*;

pub const STAT_TXRDY_1: Bitfield = Bitfield::new(0, 1);
pub const STAT_RXFIFO_READY: Bitfield = Bitfield::new(1, 1);
pub const STAT_TXRDY_2: Bitfield = Bitfield::new(2, 1);
pub const STAT_RXERR_PARITY: Bitfield = Bitfield::new(3, 1);
pub const _STAT_ACK_LEVEL: Bitfield = Bitfield::new(7, 1);
pub const STAT_IRQ: Bitfield = Bitfield::new(9, 1);
pub const STAT_TIMER: Bitfield = Bitfield::new(11, 21);

pub const _MODE_RATE_RELOADF: Bitfield = Bitfield::new(0, 2);
pub const _MODE_CHAR_LENGTH: Bitfield = Bitfield::new(2, 2);
pub const _MODE_PARITY_ENABLE: Bitfield = Bitfield::new(4, 1);
pub const _MODE_PARITY_TYPE: Bitfield = Bitfield::new(5, 1);
pub const _MODE_CLKOUT_POLARITY: Bitfield = Bitfield::new(8, 1);

pub const CTRL_TXEN: Bitfield = Bitfield::new(0, 1);
pub const _CTRL_JOYN_OUTPUT: Bitfield = Bitfield::new(1, 1);
pub const _CTRL_RXEN: Bitfield = Bitfield::new(2, 1);
pub const CTRL_ACK: Bitfield = Bitfield::new(4, 1);
pub const CTRL_RESET: Bitfield = Bitfield::new(6, 1);
pub const _CTRL_RXINT_MODE: Bitfield = Bitfield::new(8, 2);
pub const _CTRL_TXINT_ENABLE: Bitfield = Bitfield::new(10, 1);
pub const _CTRL_RXINT_ENABLE: Bitfield = Bitfield::new(11, 1);
pub const _CTRL_ACKINT_ENABLE: Bitfield = Bitfield::new(12, 1);
pub const _CTRL_JOY_SLOT: Bitfield = Bitfield::new(13, 1);

pub struct Padmc {
    pub rx_fifo: Fifo<u8>,
    pub tx_fifo: Fifo<u8>,
    pub stat: B32Register,
    pub mode: B16Register,
    pub ctrl: Ctrl,
    pub baud_reload: B16Register,
    pub padmc1040: Padmc1040,
}

impl Padmc {
    pub fn new() -> Padmc {
        Padmc {
            rx_fifo: Fifo::new(16, Some(DebugState::new("PADMC RX", true, true))),
            tx_fifo: Fifo::new(16, Some(DebugState::new("PADMC TX", true, true))),
            stat: B32Register::new(),
            mode: B16Register::new(),
            ctrl: Ctrl::new(),
            baud_reload: B16Register::new(),
            padmc1040: Padmc1040::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.padmc.padmc1040.tx_fifo = NonNull::new(&mut resources.padmc.tx_fifo as *mut Fifo<u8>);
    resources.padmc.padmc1040.rx_fifo = NonNull::new(&mut resources.padmc.rx_fifo as *mut Fifo<u8>);

    resources.r3000.memory_mapper.map(0x1F80_1040, 4, &mut resources.padmc.padmc1040 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1044, 4, &mut resources.padmc.stat as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1048, 2, &mut resources.padmc.mode as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_104A, 2, &mut resources.padmc.ctrl as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_104E, 2, &mut resources.padmc.baud_reload as *mut dyn B8MemoryMap);
}
