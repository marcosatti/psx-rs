pub mod register;

use std::ptr::NonNull;
use crate::types::register::b32_register::B32Register;
use crate::types::register::b16_register::B16Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;
use crate::resources::Resources;
use crate::resources::joystick::register::*;

pub const _STAT_TXRDY_1: Bitfield = Bitfield::new(0, 1);
pub const _STAT_RXFIFO_READY: Bitfield = Bitfield::new(1, 1);
pub const _STAT_TXRDY_2: Bitfield = Bitfield::new(2, 1);
pub const _STAT_RXERR_PARITY: Bitfield = Bitfield::new(3, 1);
pub const _STAT_ACK_LEVEL: Bitfield = Bitfield::new(7, 1);
pub const _STAT_IRQ: Bitfield = Bitfield::new(9, 1);
pub const _STAT_TIMER: Bitfield = Bitfield::new(11, 21);

pub const _MODE_RATE_RELOADF: Bitfield = Bitfield::new(0, 2);
pub const _MODE_CHAR_LENGTH: Bitfield = Bitfield::new(2, 2);
pub const _MODE_PARITY_ENABLE: Bitfield = Bitfield::new(4, 1);
pub const _MODE_PARITY_TYPE: Bitfield = Bitfield::new(5, 1);
pub const _MODE_CLKOUT_POLARITY: Bitfield = Bitfield::new(8, 1);

pub const _CTRL_TXEN: Bitfield = Bitfield::new(0, 1);
pub const _CTRL_JOYN_OUTPUT: Bitfield = Bitfield::new(1, 1);
pub const _CTRL_RXEN: Bitfield = Bitfield::new(2, 1);
pub const _CTRL_ACK: Bitfield = Bitfield::new(4, 1);
pub const _CTRL_RESET: Bitfield = Bitfield::new(6, 1);
pub const _CTRL_RXINT_MODE: Bitfield = Bitfield::new(8, 2);
pub const _CTRL_TXINT_ENABLE: Bitfield = Bitfield::new(10, 1);
pub const _CTRL_RXINT_ENABLE: Bitfield = Bitfield::new(11, 1);
pub const _CTRL_ACKINT_ENABLE: Bitfield = Bitfield::new(12, 1);
pub const _CTRL_JOY_SLOT: Bitfield = Bitfield::new(13, 1);

pub struct Joystick {
    pub rx_fifo: Fifo<u8>,
    pub tx_fifo: Fifo<u8>,
    pub stat: B32Register,
    pub mode: B16Register,
    pub ctrl: B16Register,
    pub baud_reload: B16Register,
    pub joystick1040: Joystick1040,
}

impl Joystick {
    pub fn new() -> Joystick {
        Joystick {
            rx_fifo: Fifo::new(16, Some(DebugState::new("JOYSTICK RX", true, true))),
            tx_fifo: Fifo::new(16, Some(DebugState::new("JOYSTICK TX", true, true))),
            stat: B32Register::new(),
            mode: B16Register::new(),
            ctrl: B16Register::new(),
            baud_reload: B16Register::new(),
            joystick1040: Joystick1040::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.joystick.joystick1040.tx_fifo = NonNull::new(&mut resources.joystick.tx_fifo as *mut Fifo<u8>);
    resources.joystick.joystick1040.rx_fifo = NonNull::new(&mut resources.joystick.rx_fifo as *mut Fifo<u8>);

    resources.r3000.memory_mapper.map::<u32>(0x1F80_1040, 4, &mut resources.joystick.joystick1040 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1044, 4, &mut resources.joystick.stat as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1048, 2, &mut resources.joystick.mode as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_104A, 2, &mut resources.joystick.ctrl as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_104E, 2, &mut resources.joystick.baud_reload as *mut dyn B8MemoryMap);
}
