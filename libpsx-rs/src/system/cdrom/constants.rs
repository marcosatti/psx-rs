use crate::types::bitfield::Bitfield;

pub const STATUS_INDEX: Bitfield = Bitfield::new(0, 2);
pub const _STATUS_ADPBUSY: Bitfield = Bitfield::new(2, 1);
pub const STATUS_PRMEMPT: Bitfield = Bitfield::new(3, 1);
pub const STATUS_PRMWRDY: Bitfield = Bitfield::new(4, 1);
pub const STATUS_RSLRRDY: Bitfield = Bitfield::new(5, 1);
pub const STATUS_DRQSTS: Bitfield = Bitfield::new(6, 1);
pub const STATUS_BUSYSTS: Bitfield = Bitfield::new(7, 1);
pub const INT_FLAG_CLRPRM: Bitfield = Bitfield::new(6, 1);
pub const REQUEST_SMEN: Bitfield = Bitfield::new(5, 1);
pub const REQUEST_BFWR: Bitfield = Bitfield::new(6, 1);
pub const REQUEST_BFRD: Bitfield = Bitfield::new(7, 1);
pub const INTERRUPT_FLAGS: Bitfield = Bitfield::new(0, 5);

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6; // Unknown; 33.8688 MHz
pub const VERSION: [u8; 4] = [0x94, 0x09, 0x19, 0x19];
