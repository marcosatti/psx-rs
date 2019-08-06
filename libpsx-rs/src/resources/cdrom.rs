pub mod register;

use std::ptr::NonNull;
use crate::types::bitfield::Bitfield;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::queue::Queue;
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
    pub command: Command,
    pub response: Queue<u8, 16>,
    pub parameter: Queue<u8, 16>,
    pub data: Queue<u8, 16>,
    pub int_enable: B8Register,
    pub int_flag: B8Register,
    pub request: B8Register,
    pub cdrom1801: Cdrom1801,
    pub cdrom1802: Cdrom1802,
    pub cdrom1803: Cdrom1803,
}

impl Cdrom {
    pub fn new() -> Cdrom {
        Cdrom {
            status: B8Register::new(),
            command: Command::new(),
            response: Queue::new(),
            parameter: Queue::new(),
            data: Queue::new(),
            int_enable: B8Register::new(),
            int_flag: B8Register::new(),
            request: B8Register::new(),
            cdrom1801: Cdrom1801::new(),
            cdrom1802: Cdrom1802::new(),
            cdrom1803: Cdrom1803::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.cdrom.cdrom1801.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1801.command = NonNull::new(&mut resources.cdrom.command as *mut Command);
    resources.cdrom.cdrom1801.response = NonNull::new(&mut resources.cdrom.response as *mut Queue<u8, 16>);

    resources.cdrom.cdrom1802.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1802.parameter = NonNull::new(&mut resources.cdrom.parameter as *mut Queue<u8, 16>);
    resources.cdrom.cdrom1802.data = NonNull::new(&mut resources.cdrom.data as *mut Queue<u8, 16>);
    resources.cdrom.cdrom1802.int_enable = NonNull::new(&mut resources.cdrom.int_enable as *mut B8Register);

    resources.cdrom.cdrom1803.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1803.int_flag = NonNull::new(&mut resources.cdrom.int_flag as *mut B8Register);
    resources.cdrom.cdrom1803.request = NonNull::new(&mut resources.cdrom.request as *mut B8Register);

    resources.r3000.memory_mapper.map::<u32>(0x1F80_1800, 1, &mut resources.cdrom.status as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1801, 1, &mut resources.cdrom.cdrom1801 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1802, 1, &mut resources.cdrom.cdrom1802 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1803, 1, &mut resources.cdrom.cdrom1803 as *mut dyn B8MemoryMap);
}
