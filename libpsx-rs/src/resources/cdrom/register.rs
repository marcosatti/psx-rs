use std::collections::VecDeque;
use std::ptr::NonNull;
use spin::Mutex;
use crate::types::register::b8_register::B8Register;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::resources::cdrom::STATUS_INDEX;

pub struct Cdrom1801 {
    pub status: Option<NonNull<B8Register>>,
    pub command: Command,
    pub response_mutex: Mutex<()>,
    pub response: VecDeque<u8>, 
}

impl Cdrom1801 {
    pub fn new() -> Cdrom1801 {
        Cdrom1801 {
            status: None,
            command: Command::new(),
            response_mutex: Mutex::new(()),
            response: VecDeque::new(),
        }
    }
}

impl B8MemoryMap for Cdrom1801 {
    fn read_u8(&mut self, offset: usize) -> u8 {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
    
    fn write_u8(&mut self, offset: usize, value: u8) {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => B8MemoryMap::write_u8(&mut self.command, offset, value),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
}

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
    fn read_u8(&mut self, offset: usize) -> u8 {
        B8MemoryMap::read_u8(&mut self.register, offset)
    }

    fn write_u8(&mut self, offset: usize, value: u8) {
        if self.write_latch { panic!("Write latch still on"); }
        B8MemoryMap::write_u8(&mut self.register, offset, value);
        self.write_latch = true;
    }
}

pub struct Cdrom1802 {
    pub status: Option<NonNull<B8Register>>,
    pub parameter_mutex: Mutex<()>,
    pub parameter: VecDeque<u8>, 
    pub data_mutex: Mutex<()>,
    pub data: VecDeque<u8>, 
    pub int_enable: B8Register,
}

impl Cdrom1802 {
    pub fn new() -> Cdrom1802 {
        Cdrom1802 {
            status: None,
            parameter_mutex: Mutex::new(()),
            parameter: VecDeque::new(),
            data_mutex: Mutex::new(()),
            data: VecDeque::new(),
            int_enable: B8Register::new(),
        }
    }
}

impl B8MemoryMap for Cdrom1802 {
    fn read_u8(&mut self, offset: usize) -> u8 {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
    
    fn write_u8(&mut self, offset: usize, _value: u8) {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }

    fn read_u16(&mut self, offset: usize) -> u16 {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
}

pub struct Cdrom1803 {
    pub status: Option<NonNull<B8Register>>,
    pub int_flag: B8Register,
    pub request: B8Register,
}

impl Cdrom1803 {
    pub fn new() -> Cdrom1803 {
        Cdrom1803 {
            status: None,
            int_flag: B8Register::new(),
            request: B8Register::new(),
        }
    }
}

impl B8MemoryMap for Cdrom1803 {
    fn read_u8(&mut self, offset: usize) -> u8 {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
    
    fn write_u8(&mut self, offset: usize, _value: u8) {
        if offset != 0 { panic!("Invalid offset"); }
        let index = unsafe { self.status.unwrap().as_ref().read_bitfield(STATUS_INDEX) };
        match index {
            0 => unimplemented!(),
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => unimplemented!(),
            _ => panic!("Index {} does not exist", index),
        }
    }
}
