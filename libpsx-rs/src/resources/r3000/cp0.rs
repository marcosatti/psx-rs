pub mod register;

use std::ptr::NonNull;
use crate::types::register::b32_register::B32Register;
use crate::types::bitfield::Bitfield;
use crate::resources::Resources;
use crate::resources::r3000::cp0::register::*;

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

fn prid() -> u32 {
    let mut value: u32 = 0;
    value = PRID_REVISION.insert_into(value, 2);
    value = PRID_IMPLEMENTATION.insert_into(value, 0);
    value
}

pub struct Cp0 {
    pub bpc: B32Register,
    pub bda: B32Register,
    pub jump_dest: B32Register,
    pub dcic: B32Register,
    pub bdam: B32Register,
    pub bpcm: B32Register,
    pub status: B32Register,
    pub cause: Cause,
    pub epc: B32Register,
    pub prid: B32Register,
    pub register: [Option<NonNull<B32Register>>; 64],
}

impl Cp0 {
    pub fn new() -> Cp0 {
        Cp0 {
            bpc: B32Register::new(),
            bda: B32Register::new(),
            jump_dest: B32Register::new(),
            dcic: B32Register::new(),
            bdam: B32Register::new(),
            bpcm: B32Register::new(),
            status: B32Register::new(),
            cause: Cause::new(),
            epc: B32Register::new(),
            prid: B32Register::read_only(prid()),
            register: [None; 64],
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.cp0.status.write_bitfield(STATUS_KUC, 0);
    resources.r3000.cp0.status.write_bitfield(STATUS_IEC, 0);
    resources.r3000.cp0.status.write_bitfield(STATUS_BEV, 1);
    resources.r3000.cp0.status.write_bitfield(STATUS_TS, 1);

    resources.r3000.cp0.register[3] = Some(NonNull::new(&mut resources.r3000.cp0.bpc as *mut B32Register).unwrap());
    resources.r3000.cp0.register[5] = Some(NonNull::new(&mut resources.r3000.cp0.bda as *mut B32Register).unwrap());
    resources.r3000.cp0.register[6] = Some(NonNull::new(&mut resources.r3000.cp0.jump_dest as *mut B32Register).unwrap());
    resources.r3000.cp0.register[7] = Some(NonNull::new(&mut resources.r3000.cp0.dcic as *mut B32Register).unwrap());
    resources.r3000.cp0.register[9] = Some(NonNull::new(&mut resources.r3000.cp0.bdam as *mut B32Register).unwrap());
    resources.r3000.cp0.register[11] = Some(NonNull::new(&mut resources.r3000.cp0.bpcm as *mut B32Register).unwrap());
    resources.r3000.cp0.register[12] = Some(NonNull::new(&mut resources.r3000.cp0.status as *mut B32Register).unwrap());
    resources.r3000.cp0.register[13] = Some(NonNull::new(&mut resources.r3000.cp0.cause.register as *mut B32Register).unwrap());
    resources.r3000.cp0.register[14] = Some(NonNull::new(&mut resources.r3000.cp0.epc as *mut B32Register).unwrap());
    resources.r3000.cp0.register[15] = Some(NonNull::new(&mut resources.r3000.cp0.prid as *mut B32Register).unwrap());
}
