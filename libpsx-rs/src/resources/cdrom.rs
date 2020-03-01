pub mod register;

use std::ptr::NonNull;
use std::collections::VecDeque;
use crate::types::bitfield::Bitfield;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;
use crate::resources::Resources;
use crate::resources::cdrom::register::*;

pub const STATUS_INDEX: Bitfield = Bitfield::new(0, 2);
pub const _STATUS_ADPBUSY: Bitfield = Bitfield::new(2, 1);
pub const STATUS_PRMEMPT: Bitfield = Bitfield::new(3, 1);
pub const STATUS_PRMWRDY: Bitfield = Bitfield::new(4, 1);
pub const STATUS_RSLRRDY: Bitfield = Bitfield::new(5, 1);
pub const _STATUS_DRQSTS: Bitfield = Bitfield::new(6, 1);
pub const STATUS_BUSYSTS: Bitfield = Bitfield::new(7, 1);

pub const INT_FLAG_CLRPRM: Bitfield = Bitfield::new(6, 1);

pub const INTERRUPT_FLAGS: Bitfield = Bitfield::new(0, 5);

pub struct Cdrom {
    pub status: B8Register,
    pub command: Command,
    pub response: Fifo<u8>,
    pub parameter: Fifo<u8>,
    pub data: Fifo<u8>,
    pub int_enable: IntEnable,
    pub int_flag: IntFlag,
    pub request: B8Register,
    pub cdrom1801: Cdrom1801,
    pub cdrom1802: Cdrom1802,
    pub cdrom1803: Cdrom1803,

    pub command_index: Option<u8>,
    pub command_iteration: usize,

    /// Current LBA address.
    pub lba_address: usize, 
    /// Reading status.
    pub reading: bool,
    pub read_buffer: VecDeque<u8>,
}

impl Cdrom {
    pub fn new() -> Cdrom {
        Cdrom {
            status: B8Register::new(),
            command: Command::new(),
            response: Fifo::new(16, Some(DebugState::new("CDROM RESPONSE", true, true))),
            parameter: Fifo::new(16, Some(DebugState::new("CDROM PARAMETER", true, true))),
            data: Fifo::new(16, Some(DebugState::new("CDROM DATA", true, true))),
            int_enable: IntEnable::new(),
            int_flag: IntFlag::new(),
            request: B8Register::new(),
            cdrom1801: Cdrom1801::new(),
            cdrom1802: Cdrom1802::new(),
            cdrom1803: Cdrom1803::new(),
            command_index: None,
            command_iteration: 0,
            lba_address: 0,
            reading: false,
            read_buffer: VecDeque::with_capacity(2048),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.cdrom.int_enable.register.write_u8(0xE0);
    resources.cdrom.int_flag.register.write_u8(0xE0);

    resources.cdrom.cdrom1801.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1801.command = NonNull::new(&mut resources.cdrom.command as *mut Command);
    resources.cdrom.cdrom1801.response = NonNull::new(&mut resources.cdrom.response as *mut Fifo<u8>);

    resources.cdrom.cdrom1802.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1802.parameter = NonNull::new(&mut resources.cdrom.parameter as *mut Fifo<u8>);
    resources.cdrom.cdrom1802.data = NonNull::new(&mut resources.cdrom.data as *mut Fifo<u8>);
    resources.cdrom.cdrom1802.int_enable = NonNull::new(&mut resources.cdrom.int_enable as *mut IntEnable);

    resources.cdrom.cdrom1803.status = NonNull::new(&mut resources.cdrom.status as *mut B8Register);
    resources.cdrom.cdrom1803.int_flag = NonNull::new(&mut resources.cdrom.int_flag as *mut IntFlag);
    resources.cdrom.cdrom1803.request = NonNull::new(&mut resources.cdrom.request as *mut B8Register);

    resources.r3000.memory_mapper.map(0x1F80_1800, 1, &mut resources.cdrom.status as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1801, 1, &mut resources.cdrom.cdrom1801 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1802, 1, &mut resources.cdrom.cdrom1802 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1803, 1, &mut resources.cdrom.cdrom1803 as *mut dyn B8MemoryMap);
}
