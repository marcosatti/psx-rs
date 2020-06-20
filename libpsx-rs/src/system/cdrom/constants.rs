use crate::types::bitfield::Bitfield;

pub(crate) const STATUS_INDEX: Bitfield = Bitfield::new(0, 2);
pub(crate) const _STATUS_ADPBUSY: Bitfield = Bitfield::new(2, 1);
pub(crate) const STATUS_PRMEMPT: Bitfield = Bitfield::new(3, 1);
pub(crate) const STATUS_PRMWRDY: Bitfield = Bitfield::new(4, 1);
pub(crate) const STATUS_RSLRRDY: Bitfield = Bitfield::new(5, 1);
pub(crate) const STATUS_DRQSTS: Bitfield = Bitfield::new(6, 1);
pub(crate) const _STATUS_BUSYSTS: Bitfield = Bitfield::new(7, 1);
pub(crate) const INT_FLAG_CLRPRM: Bitfield = Bitfield::new(6, 1);
pub(crate) const REQUEST_SMEN: Bitfield = Bitfield::new(5, 1);
pub(crate) const _REQUEST_BFWR: Bitfield = Bitfield::new(6, 1);
pub(crate) const REQUEST_BFRD: Bitfield = Bitfield::new(7, 1);
pub(crate) const INTERRUPT_FLAGS: Bitfield = Bitfield::new(0, 5);

pub(crate) const CLOCK_SPEED: f64 = 33.8688 * 1e6; // Unknown; 33.8688 MHz
pub(crate) const CLOCK_SPEED_PERIOD: f64 = 1.0 / CLOCK_SPEED;
pub(crate) const VERSION: [u8; 4] = [0x94, 0x09, 0x19, 0x19];
pub(crate) const SECTOR_DELAY_CYCLES_SINGLE_SPEED: usize = 0x6E1CD;
