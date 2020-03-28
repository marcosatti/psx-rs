use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::*;
use crate::types::fifo::Fifo;
use crate::types::bitfield::Bitfield;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::fifo::debug::DebugState;

pub struct Command {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl Command {
    pub fn new() -> Command {
        Command {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for Command {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u8(&mut self.register, offset, value)
    }
}

pub struct IntEnable {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl IntEnable {
    pub fn new() -> IntEnable {
        IntEnable {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for IntEnable {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        let mut register_value = self.register.read_u8();
        register_value = INTERRUPT_FLAGS.insert_into(register_value, value);
        B8MemoryMap::write_u8(&mut self.register, offset, register_value)
    }
}

pub struct Request {
    pub register: B8Register,
    pub write_latch: AtomicBool,
}

impl Request {
    pub fn new() -> Request {
        Request {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
        }
    }
}

impl B8MemoryMap for Request {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        //assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u8(&mut self.register, offset, value)
    }
}

pub struct IntFlag {
    pub register: B8Register,
    pub write_latch: AtomicBool,
    pub parameter_reset: AtomicBool,
}

impl IntFlag {
    pub fn new() -> IntFlag {
        IntFlag {
            register: B8Register::new(),
            write_latch: AtomicBool::new(false),
            parameter_reset: AtomicBool::new(false),
        }
    }

    pub fn set_interrupt(&mut self, line: usize) {
        assert!(line <= 10, "Invalid interrupt index");
        let value = self.register.read_u8() | (line as u8);
        self.register.write_u8(value);
    }
}

impl B8MemoryMap for IntFlag {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still pending");
        assert!(!self.parameter_reset.load(Ordering::Acquire), "Parameter FIFO reset still pending");
        self.write_latch.store(true, Ordering::Release);

        if INT_FLAG_CLRPRM.extract_from(value) != 0 {
            self.parameter_reset.store(true, Ordering::Release);
        }

        let mut register_value = self.register.read_u8();
        register_value = INTERRUPT_FLAGS.acknowledge(register_value, value);
        register_value = Bitfield::new(5, 1).insert_into(register_value, 1);
        register_value = Bitfield::new(6, 1).insert_into(register_value, 1);
        register_value = Bitfield::new(7, 1).insert_into(register_value, 1);

        B8MemoryMap::write_u8(&mut self.register, offset, register_value)
    }
}

pub struct Cdrom1801 {
    pub status: Option<NonNull<B8Register>>,
    pub command: Option<NonNull<Command>>,
    pub response: Option<NonNull<Fifo<u8>>>,
}

impl Cdrom1801 {
    pub fn new() -> Cdrom1801 {
        Cdrom1801 {
            status: None,
            command: None,
            response: None,
        }
    }
}

impl B8MemoryMap for Cdrom1801 {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => unimplemented!(),
                1 => self.response.as_ref().unwrap().as_ref().read_one().map_err(|_| ReadError::Empty),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => B8MemoryMap::write_u8(self.command.as_mut().unwrap().as_mut(), offset, value),
                1 => unimplemented!(),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
}

pub struct Cdrom1802 {
    pub status: Option<NonNull<B8Register>>,
    pub parameter: Option<NonNull<Fifo<u8>>>,
    pub data: Option<NonNull<Fifo<u8>>>,
    pub int_enable: Option<NonNull<IntEnable>>,
}

impl Cdrom1802 {
    pub fn new() -> Cdrom1802 {
        Cdrom1802 {
            status: None,
            parameter: None,
            data: None,
            int_enable: None
        }
    }
}

impl B8MemoryMap for Cdrom1802 {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => unimplemented!(),
                1 => unimplemented!(),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => self.parameter.as_mut().unwrap().as_mut().write_one(value).map_err(|_| WriteError::Full),
                1 => B8MemoryMap::write_u8(self.int_enable.as_mut().unwrap().as_mut(), offset, value),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
}

pub struct Cdrom1803 {
    pub status: Option<NonNull<B8Register>>,
    pub int_enable: Option<NonNull<IntEnable>>,
    pub int_flag: Option<NonNull<IntFlag>>,
    pub request: Option<NonNull<Request>>,
}

impl Cdrom1803 {
    pub fn new() -> Cdrom1803 {
        Cdrom1803 {
            status: None,
            int_enable: None,
            int_flag: None,
            request: None,
        }
    }
}

impl B8MemoryMap for Cdrom1803 {
    fn read_u8(&mut self, offset: u32) -> ReadResult<u8> {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => B8MemoryMap::read_u8(self.int_enable.as_mut().unwrap().as_mut(), offset),
                1 => B8MemoryMap::read_u8(self.int_flag.as_mut().unwrap().as_mut(), offset),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
    
    fn write_u8(&mut self, offset: u32, value: u8) -> WriteResult {
        unsafe { 
            assert!(offset == 0, "Invalid offset");
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => B8MemoryMap::write_u8(self.request.as_mut().unwrap().as_mut(), offset, value),
                1 => B8MemoryMap::write_u8(self.int_flag.as_mut().unwrap().as_mut(), offset, value),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
}

pub struct Cdrom {
    pub status: B8Register,
    pub command: Command,
    pub response: Fifo<u8>,
    pub parameter: Fifo<u8>,
    pub data: Fifo<u8>,
    pub int_enable: IntEnable,
    pub int_flag: IntFlag,
    pub request: Request,
    pub cdrom1801: Cdrom1801,
    pub cdrom1802: Cdrom1802,
    pub cdrom1803: Cdrom1803,

    /// Command state.
    pub command_index: Option<u8>,
    pub command_iteration: usize,

    /// Pausing status.
    pub pausing: bool,
    /// Seeking status.
    pub seeking: bool,
    /// Reading status.
    pub reading: bool,
    pub read_buffer: VecDeque<u8>,
    /// Current LBA address.
    pub lba_address: usize, 
}

impl Cdrom {
    pub fn new() -> Cdrom {
        Cdrom {
            status: B8Register::new(),
            command: Command::new(),
            response: Fifo::new(16, Some(DebugState::new("CDROM RESPONSE", true, true))),
            parameter: Fifo::new(16, Some(DebugState::new("CDROM PARAMETER", true, true))),
            data: Fifo::new(64, Some(DebugState::new("CDROM DATA", true, true))),
            int_enable: IntEnable::new(),
            int_flag: IntFlag::new(),
            request: Request::new(),
            cdrom1801: Cdrom1801::new(),
            cdrom1802: Cdrom1802::new(),
            cdrom1803: Cdrom1803::new(),
            command_index: None,
            command_iteration: 0,
            pausing: false,
            seeking: false,
            reading: false,
            read_buffer: VecDeque::with_capacity(2048),
            lba_address: 0,
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
    resources.cdrom.cdrom1803.int_enable = NonNull::new(&mut resources.cdrom.int_enable as *mut IntEnable);
    resources.cdrom.cdrom1803.int_flag = NonNull::new(&mut resources.cdrom.int_flag as *mut IntFlag);
    resources.cdrom.cdrom1803.request = NonNull::new(&mut resources.cdrom.request as *mut Request);

    resources.r3000.memory_mapper.map(0x1F80_1800, 1, &mut resources.cdrom.status as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1801, 1, &mut resources.cdrom.cdrom1801 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1802, 1, &mut resources.cdrom.cdrom1802 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1803, 1, &mut resources.cdrom.cdrom1803 as *mut dyn B8MemoryMap);
}