use std::ptr::NonNull;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::*;
use crate::types::fifo::Fifo;
use crate::resources::cdrom::*;

pub struct Command {
    pub register: B8Register,
    pub write_latch: bool,
}

impl Command {
    pub fn new() -> Command {
        Command {
            register: B8Register::new(),
            write_latch: false,
        }
    }
}

impl B8MemoryMap for Command {
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        if self.write_latch { panic!("Write latch still on"); }
        B8MemoryMap::write_u8(&mut self.register, offset, value).unwrap();
        self.write_latch = true;
        Ok(())
    }
}

pub struct IntEnable {
    pub register: B8Register,
}

impl IntEnable {
    pub fn new() -> IntEnable {
        IntEnable {
            register: B8Register::with_value(0xE0),
        }
    }
}

impl B8MemoryMap for IntEnable {
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        let value = INTERRUPT_FLAGS.insert_into(self.register.read_u8(), value);
        B8MemoryMap::write_u8(&mut self.register, offset, value)
    }
}

pub struct IntFlag {
    pub register: B8Register,
    pub parameter_reset: bool,
}

impl IntFlag {
    pub fn new() -> IntFlag {
        IntFlag {
            register: B8Register::with_value(0xE0),
            parameter_reset: false,
        }
    }

    pub fn set_interrupt(&mut self, index: u8) {
        if index > 10 {
            panic!("Invalid interrupt index");
        }

        let value = self.register.read_u8() | index;
        self.register.write_u8(value);
    }
}

impl B8MemoryMap for IntFlag {
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        if self.parameter_reset { panic!("Parameter FIFO reset still pending"); }

        if INT_FLAG_CLRPRM.extract_from(value) != 0 {
            self.parameter_reset = true;
        }

        let register_value = self.register.read_u8();
        let value = INTERRUPT_FLAGS.acknowledge(register_value, value);
        B8MemoryMap::write_u8(&mut self.register, offset, value).unwrap();

        Ok(())
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
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
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
    
    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
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
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
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
    
    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
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
    pub int_flag: Option<NonNull<IntFlag>>,
    pub request: Option<NonNull<B8Register>>,
}

impl Cdrom1803 {
    pub fn new() -> Cdrom1803 {
        Cdrom1803 {
            status: None,
            int_flag: None,
            request: None,
        }
    }
}

impl B8MemoryMap for Cdrom1803 {
    fn read_u8(&mut self, offset: usize) -> ReadResult<u8> {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => unimplemented!(),
                1 => B8MemoryMap::read_u8(self.int_flag.as_mut().unwrap().as_mut(), offset),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
    
    fn write_u8(&mut self, offset: usize, value: u8) -> WriteResult {
        unsafe { 
            if offset != 0 { panic!("Invalid offset"); }
            let index = self.status.as_ref().unwrap().as_ref().read_bitfield(STATUS_INDEX);
            match index {
                0 => unimplemented!(),
                1 => B8MemoryMap::write_u8(self.int_flag.as_mut().unwrap().as_mut(), offset, value),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
}
