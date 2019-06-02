pub mod register;

use std::ptr::NonNull;
use crate::types::bitfield::Bitfield;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::resources::Resources;
use crate::resources::cdrom::register::*;

static STATUS_INDEX: Bitfield = Bitfield::new(0, 2);
static _STATUS_ADPBUSY: Bitfield = Bitfield::new(2, 1);
static _STATUS_PRMEMPT: Bitfield = Bitfield::new(3, 1);
static _STATUS_PRMWRDY: Bitfield = Bitfield::new(4, 1);
static _STATUS_RSLRRDY: Bitfield = Bitfield::new(5, 1);
static _STATUS_DRQSTS: Bitfield = Bitfield::new(6, 1);
static _STATUS_BUSYSTS: Bitfield = Bitfield::new(7, 1);

pub struct Cdrom {
    pub status: B8Register,
    pub cdrom1801: Cdrom1801,
    pub cdrom1802: Cdrom1802,
    pub cdrom1803: Cdrom1803,
}

impl Cdrom {
    pub fn new() -> Cdrom {
        Cdrom {
            status: B8Register::new(),
            cdrom1801: Cdrom1801::new(),
            cdrom1802: Cdrom1802::new(),
            cdrom1803: Cdrom1803::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.cdrom.cdrom1801.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1802.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1803.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);

    resources.r3000.memory_mapper.map::<u32>(0x1F80_1800, 1, &mut resources.cdrom.status as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1801, 1, &mut resources.cdrom.cdrom1801 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1802, 1, &mut resources.cdrom.cdrom1802 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1803, 1, &mut resources.cdrom.cdrom1803 as *mut dyn B8MemoryMap);
}
