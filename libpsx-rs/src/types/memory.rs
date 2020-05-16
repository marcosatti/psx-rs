mod edge;
mod level;

use crate::types::bitfield::Bitfield;
pub use edge::*;
pub use level::*;
use std::cell::UnsafeCell;

// TODO: const generics once available.

/// Shared memory
/// No synchronisation is provided, it must be done through other means.
pub struct B8Memory {
    memory: UnsafeCell<Vec<u8>>,
}

impl B8Memory {
    pub fn new(size: usize) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(vec![0; size]),
        }
    }

    pub fn new_initialized(size: usize, value: u8) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(vec![value; size]),
        }
    }

    pub fn read_raw(&self, byte_offset: u32) -> &[u8] {
        unsafe { &(*self.memory.get())[byte_offset as usize..] }
    }

    pub fn write_raw(&self, byte_offset: u32, data: &[u8]) {
        unsafe {
            (*self.memory.get())[byte_offset as usize..(byte_offset as usize) + data.len()].copy_from_slice(data);
        }
    }

    pub fn read_u8(&self, byte_offset: u32) -> u8 {
        unsafe { (*self.memory.get())[byte_offset as usize] }
    }

    pub fn write_u8(&self, byte_offset: u32, value: u8) {
        unsafe {
            (*self.memory.get())[byte_offset as usize] = value;
        }
    }

    pub fn read_u16(&self, byte_offset: u32) -> u16 {
        assert_eq!(byte_offset % 2, 0);
        unsafe { *((&(*self.memory.get())[byte_offset as usize] as *const u8) as *const u16) }
    }

    pub fn write_u16(&self, byte_offset: u32, value: u16) {
        assert_eq!(byte_offset % 2, 0);
        unsafe {
            *((&mut (*self.memory.get())[byte_offset as usize] as *mut u8) as *mut u16) = value;
        }
    }

    pub fn read_u32(&self, byte_offset: u32) -> u32 {
        assert_eq!(byte_offset % 4, 0);
        unsafe { *((&(*self.memory.get())[byte_offset as usize] as *const u8) as *const u32) }
    }

    pub fn write_u32(&self, byte_offset: u32, value: u32) {
        assert_eq!(byte_offset % 4, 0);
        unsafe {
            *((&mut (*self.memory.get())[byte_offset as usize] as *mut u8) as *mut u32) = value;
        }
    }

    pub fn read_bitfield_u8(&self, byte_offset: u32, bitfield: Bitfield) -> u8 {
        bitfield.extract_from(self.read_u8(byte_offset))
    }

    pub fn write_bitfield_u8(&self, byte_offset: u32, bitfield: Bitfield, value: u8) {
        self.write_u8(byte_offset, bitfield.insert_into(self.read_u8(byte_offset), value));
    }

    pub fn read_bitfield_u16(&self, byte_offset: u32, bitfield: Bitfield) -> u16 {
        bitfield.extract_from(self.read_u16(byte_offset))
    }

    pub fn write_bitfield_u16(&self, byte_offset: u32, bitfield: Bitfield, value: u16) {
        self.write_u16(byte_offset, bitfield.insert_into(self.read_u16(byte_offset), value));
    }

    pub fn read_bitfield_u32(&self, byte_offset: u32, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32(byte_offset))
    }

    pub fn write_bitfield_u32(&self, byte_offset: u32, bitfield: Bitfield, value: u32) {
        self.write_u32(byte_offset, bitfield.insert_into(self.read_u32(byte_offset), value));
    }
}

unsafe impl Send for B8Memory {
}

unsafe impl Sync for B8Memory {
}

#[repr(C)]
#[derive(Copy, Clone)]
union B32Register_ {
    v32: u32,
    v16: [u16; 2],
    v8: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
union B16Register_ {
    v16: u16,
    v8: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
union B8Register_ {
    v8: u8,
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

#[test]
fn test_bitfield() {
    let m = B8Memory::new(4);

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x11);
    m.write_u8(2, 0x22);
    m.write_u8(3, 0x33);

    let bitfield = Bitfield::new(9, 4);

    assert_eq!(m.read_bitfield_u32(0, bitfield), 8);
}
