use std::cell::UnsafeCell;
use smallvec::*;

pub struct B8Memory {
    memory: UnsafeCell<SmallVec<[u8; 4]>>,
}

impl B8Memory {
    pub fn new(size: usize) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(smallvec![0; size]),
        }
    }

    pub fn new_initialized(size: usize, value: u8) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(smallvec![value; size]),
        }
    }

    pub fn read_raw(&self, offset: u32) -> &[u8] {
        unsafe {
            &(*self.memory.get())[offset as usize..]
        }
    }

    pub fn write_raw(&mut self, data: &[u8], offset: u32) {
        unsafe {
            (*self.memory.get())[offset as usize..(offset as usize) + data.len()].copy_from_slice(data);
        }
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe {
            (*self.memory.get())[offset as usize]
        }
    }

    pub fn write_u8(&mut self, offset: u32, value: u8) {
        unsafe {
            (*self.memory.get())[offset as usize] = value;
        }
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        assert_eq!(offset % 2, 0);
        unsafe { 
            *((&(*self.memory.get())[offset as usize] as *const u8) as *const u16) 
        }
    }

    pub fn write_u16(&mut self, offset: u32, value: u16) {
        assert_eq!(offset % 2, 0);
        unsafe {
            *((&mut (*self.memory.get())[offset as usize] as *mut u8) as *mut u16) = value;
        }
    }

    pub fn read_u32(&self, offset: u32) -> u32 {
        assert_eq!(offset % 4, 0);
        unsafe { 
            *((&(*self.memory.get())[offset as usize] as *const u8) as *const u32) 
        }
    }

    pub fn write_u32(&mut self, offset: u32, value: u32) {
        assert_eq!(offset % 4, 0);
        unsafe {
            *((&mut (*self.memory.get())[offset as usize] as *mut u8) as *mut u32) = value;
        }
    }
}

#[test]
fn test_layout() {
    let m = B8Memory::new(4);

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x11);
    m.write_u8(2, 0x22);
    m.write_u8(3, 0x33);

    assert_eq!(m.read_u8(0), 0x00);
    assert_eq!(m.read_u8(1), 0x11);
    assert_eq!(m.read_u8(2), 0x22);
    assert_eq!(m.read_u8(3), 0x33);

    assert_eq!(m.read_u16(0), 0x1100);
    assert_eq!(m.read_u16(2), 0x3322);

    assert_eq!(m.read_u32(0), 0x33221100);
}
