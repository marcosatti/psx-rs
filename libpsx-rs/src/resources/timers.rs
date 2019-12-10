pub mod register;

use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::resources::Resources;
use crate::resources::timers::register::*;
use crate::types::register::b32_register::B32Register;
use crate::types::bitfield::Bitfield;

pub const MODE_SYNC_EN: Bitfield = Bitfield::new(0, 1);
pub const MODE_SYNC_MODE: Bitfield = Bitfield::new(1, 2);
pub const MODE_RESET: Bitfield = Bitfield::new(3, 1);
pub const MODE_IRQ_TARGET: Bitfield = Bitfield::new(4, 1);
pub const MODE_IRQ_OVERFLOW: Bitfield = Bitfield::new(5, 1);
pub const _MODE_IRQ_REPEAT: Bitfield = Bitfield::new(6, 1);
pub const _MODE_IRQ_PULSE: Bitfield = Bitfield::new(7, 1);
pub const MODE_CLK_SRC: Bitfield = Bitfield::new(8, 2);
pub const MODE_IRQ_STATUS: Bitfield = Bitfield::new(10, 1);
pub const MODE_TARGET_HIT: Bitfield = Bitfield::new(11, 1);
pub const MODE_OVERFLOW_HIT: Bitfield = Bitfield::new(12, 1);

pub struct Timers {
    pub timer0_count: B32Register,
    pub timer0_mode: Mode,
    pub timer0_target: B32Register,

    pub timer1_count: B32Register,
    pub timer1_mode: Mode,
    pub timer1_target: B32Register,

    pub timer2_count: B32Register,
    pub timer2_mode: Mode,
    pub timer2_target: B32Register,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            timer0_count: B32Register::new(),
            timer0_mode: Mode::new(),
            timer0_target: B32Register::new(),
            timer1_count: B32Register::new(),
            timer1_mode: Mode::new(),
            timer1_target: B32Register::new(),
            timer2_count: B32Register::new(),
            timer2_mode: Mode::new(),
            timer2_target: B32Register::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1F80_1100, 4, &mut resources.timers.timer0_count as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1104, 4, &mut resources.timers.timer0_mode as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1108, 4, &mut resources.timers.timer0_target as *mut dyn B8MemoryMap);

    resources.r3000.memory_mapper.map(0x1F80_1110, 4, &mut resources.timers.timer1_count as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1114, 4, &mut resources.timers.timer1_mode as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1118, 4, &mut resources.timers.timer1_target as *mut dyn B8MemoryMap);

    resources.r3000.memory_mapper.map(0x1F80_1120, 4, &mut resources.timers.timer2_count as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1124, 4, &mut resources.timers.timer2_mode as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1128, 4, &mut resources.timers.timer2_target as *mut dyn B8MemoryMap);
}
