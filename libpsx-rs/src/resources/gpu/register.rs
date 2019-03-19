use spin::Mutex;
use std::collections::VecDeque;
use log::debug;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::B8MemoryMap;

pub struct Gpu1810 {
    pub gp0_mutex: Mutex<()>,
    pub gp0: VecDeque<u32>, 
    pub read_mutex: Mutex<()>,
    pub read: VecDeque<u32>, 
}

impl Gpu1810 {
    pub fn new() -> Gpu1810 {
        Gpu1810 {
            gp0_mutex: Mutex::new(()),
            gp0: VecDeque::with_capacity(1024),
            read_mutex: Mutex::new(()),
            read: VecDeque::with_capacity(1024),
        }
    }
}

impl B8MemoryMap for Gpu1810 {
    fn read_u32(&mut self, offset: usize) -> u32 {
        if offset != 0 { panic!("Invalid offset"); }
        
        if self.read.len() > 0 {
            let _lock = self.read_mutex.lock();
            self.read.pop_front().unwrap()
        } else {
            debug!("GPUREAD is empty - returning 0");
            0
        }
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) {
        if offset != 0 { panic!("Invalid offset"); }
        let _lock = self.gp0_mutex.lock();
        self.gp0.push_back(value);
    }
}

pub struct Gpu1814 {
    pub gp1_lock: Mutex<()>,
    pub gp1: VecDeque<u32>, 
    pub stat: B32Register,
}

impl Gpu1814 {
    pub fn new() -> Gpu1814 {
        Gpu1814 {
            gp1_lock: Mutex::new(()),
            gp1: VecDeque::with_capacity(1024), // Not really a FIFO? But emulator needs to buffer commands.
            stat: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Gpu1814 {
    fn read_u32(&mut self, offset: usize) -> u32 {
        B8MemoryMap::read_u32(&mut self.stat, offset)
    }
    
    fn write_u32(&mut self, offset: usize, value: u32) {
        if offset != 0 { panic!("Invalid offset"); }
        let _lock = self.gp1_lock.lock();
        self.gp1.push_back(value);
    }
}
