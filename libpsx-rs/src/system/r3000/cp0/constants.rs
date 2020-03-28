use crate::types::bitfield::Bitfield;

pub const PRID_REVISION: Bitfield = Bitfield::new(0, 8);
pub const PRID_IMPLEMENTATION: Bitfield = Bitfield::new(8, 8);

pub const STATUS_IEC: Bitfield = Bitfield::new(0, 1);
pub const STATUS_KUC: Bitfield = Bitfield::new(1, 1);
pub const _STATUS_IEP: Bitfield = Bitfield::new(2, 1);
pub const _STATUS_KUP: Bitfield = Bitfield::new(3, 1);
pub const _STATUS_IEO: Bitfield = Bitfield::new(4, 1);
pub const _STATUS_KUO: Bitfield = Bitfield::new(5, 1);
pub const STATUS_IM: Bitfield = Bitfield::new(8, 8);
pub const STATUS_ISC: Bitfield = Bitfield::new(16, 1);
pub const STATUS_TS: Bitfield = Bitfield::new(21, 1);
pub const STATUS_BEV: Bitfield = Bitfield::new(22, 1);
pub const _STATUS_CU0: Bitfield = Bitfield::new(28, 1);
pub const _STATUS_CU2: Bitfield = Bitfield::new(30, 1);

pub const CAUSE_EXCCODE: Bitfield = Bitfield::new(2, 5);
pub const CAUSE_IP: Bitfield = Bitfield::new(8, 8);
pub const _CAUSE_CE: Bitfield = Bitfield::new(28, 2);
pub const CAUSE_BD: Bitfield = Bitfield::new(31, 1);

pub const CAUSE_EXCCODE_INT: usize = 0;
pub const CAUSE_EXCCODE_SYSCALL: usize = 8;

pub const CAUSE_IP_INTC: Bitfield = Bitfield::new(10, 1);
pub const _CAUSE_IP_INTC_OFFSET: Bitfield = Bitfield::new(2, 1);
