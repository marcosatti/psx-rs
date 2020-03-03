use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::*;
use crate::types::fifo::Fifo;
use crate::resources::cdrom::*;

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
