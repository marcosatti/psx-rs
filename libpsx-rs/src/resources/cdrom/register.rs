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
                0 => {
                    let command = self.command.as_mut().unwrap().as_mut();
                    B8MemoryMap::write_u8(command, offset, value)
                },
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
    pub int_enable: Option<NonNull<B8Register>>,
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
                1 => Ok(self.int_enable.as_mut().unwrap().as_mut().write_u8(value)),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }

    fn read_u16(&mut self, offset: usize) -> ReadResult<u16> {
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
}

pub struct Cdrom1803 {
    pub status: Option<NonNull<B8Register>>,
    pub int_flag: Option<NonNull<B8Register>>,
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
                0 => unimplemented!(),
                1 => Ok(self.int_flag.as_mut().unwrap().as_mut().write_u8(value)),
                2 => unimplemented!(),
                3 => unimplemented!(),
                _ => panic!("Index {} does not exist", index),
            }
        }
    }
}
