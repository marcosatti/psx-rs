use crate::types::register::b32_register::B32Register;
use crate::types::bitfield::Bitfield;

pub const _MODE_SYNC_EN: Bitfield = Bitfield::new(0, 1);
pub const _MODE_SYNC_MODE: Bitfield = Bitfield::new(1, 2);
pub const _MODE_RESET: Bitfield = Bitfield::new(3, 1);
pub const _MODE_IRQ_TARGET: Bitfield = Bitfield::new(4, 1);
pub const _MODE_IRQ_MAX: Bitfield = Bitfield::new(5, 1);
pub const _MODE_IRQ_REPEAT: Bitfield = Bitfield::new(6, 1);
pub const _MODE_IRQ_PULSE: Bitfield = Bitfield::new(7, 1);
pub const _MODE_CLK_SRC: Bitfield = Bitfield::new(8, 2);
pub const _MODE_IRQ_STATUS: Bitfield = Bitfield::new(10, 1);
pub const _MODE_TARGET_HIT: Bitfield = Bitfield::new(11, 1);
pub const _MODE_MAX_HIT: Bitfield = Bitfield::new(12, 1);

pub struct Timer {
    pub count: B32Register,
    pub mode: B32Register,
    pub target: B32Register,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            count: B32Register::new(),
            mode: B32Register::new(),
            target: B32Register::new(),
        }
    }
}
