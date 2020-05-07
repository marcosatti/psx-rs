//! Level-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master/slave access combination (ie: CPU and peripheral). 
//! No error handling is required for level-triggered conditions, therefore read/writes always succeed.

use super::{
    B8Register_,
    B16Register_,
    B32Register_,
};
use crate::types::bitfield::Bitfield;
use std::cell::UnsafeCell;

pub struct B32LevelRegister {
    memory: UnsafeCell<B32Register_>,
}

impl B32LevelRegister {
    pub fn new() -> B32LevelRegister {
        B32LevelRegister {
            memory: UnsafeCell::new(B32Register_ {
                v32: 0,
            }),
        }
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe { (*self.memory.get()).v8[offset as usize] }
    }

    pub fn write_u8(&self, offset: u32, value: u8) {
        unsafe {
            (*self.memory.get()).v8[offset as usize] = value;
        }
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        unsafe { (*self.memory.get()).v16[offset as usize] }
    }

    pub fn write_u16(&self, offset: u32, value: u16) {
        unsafe {
            (*self.memory.get()).v16[offset as usize] = value;
        }
    }

    pub fn read_u32(&self) -> u32 {
        unsafe { (*self.memory.get()).v32 }
    }

    pub fn write_u32(&self, value: u32) {
        unsafe {
            (*self.memory.get()).v32 = value;
        }
    }

    pub fn read_bitfield(&self, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32())
    }

    pub fn write_bitfield(&self, bitfield: Bitfield, value: u32) {
        self.write_u32(bitfield.insert_into(self.read_u32(), value));
    }
}

unsafe impl Send for B32LevelRegister {
}

unsafe impl Sync for B32LevelRegister {
}

pub struct B16LevelRegister {
    memory: UnsafeCell<B16Register_>,
}

impl B16LevelRegister {
    pub fn new() -> B16LevelRegister {
        B16LevelRegister {
            memory: UnsafeCell::new(B16Register_ {
                v16: 0,
            }),
        }
    }

    pub fn read_u8(&self, offset: u32) -> u8 {
        unsafe { (*self.memory.get()).v8[offset as usize] }
    }

    pub fn write_u8(&self, offset: u32, value: u8) {
        unsafe {
            (*self.memory.get()).v8[offset as usize] = value;
        }
    }

    pub fn read_u16(&self) -> u16 {
        unsafe { (*self.memory.get()).v16 }
    }

    pub fn write_u16(&self, value: u16) {
        unsafe {
            (*self.memory.get()).v16 = value;
        }
    }

    pub fn read_bitfield(&self, bitfield: Bitfield) -> u16 {
        bitfield.extract_from(self.read_u16())
    }

    pub fn write_bitfield(&self, bitfield: Bitfield, value: u16) {
        self.write_u16(bitfield.insert_into(self.read_u16(), value));
    }
}

unsafe impl Send for B16LevelRegister {
}

unsafe impl Sync for B16LevelRegister {
}

pub struct B8LevelRegister {
    memory: UnsafeCell<B8Register_>,
}

impl B8LevelRegister {
    pub fn new() -> B8LevelRegister {
        B8LevelRegister {
            memory: UnsafeCell::new(B8Register_ {
                v8: 0,
            }),
        }
    }

    pub fn read_u8(&self) -> u8 {
        unsafe { (*self.memory.get()).v8 }
    }

    pub fn write_u8(&self, value: u8) {
        unsafe {
            (*self.memory.get()).v8 = value;
        }
    }

    pub fn read_bitfield(&self, bitfield: Bitfield) -> u8 {
        bitfield.extract_from(self.read_u8())
    }

    pub fn write_bitfield(&self, bitfield: Bitfield, value: u8) {
        self.write_u8(bitfield.insert_into(self.read_u8(), value));
    }
}

unsafe impl Send for B8LevelRegister {
}

unsafe impl Sync for B8LevelRegister {
}

#[test]
fn test_layout_32() {
    let m = B32LevelRegister::new();

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x33);
    m.write_u8(2, 0x22);
    m.write_u8(3, 0x11);

    assert_eq!(m.read_u8(0), 0x00);
    assert_eq!(m.read_u8(1), 0x33);
    assert_eq!(m.read_u8(2), 0x22);
    assert_eq!(m.read_u8(3), 0x11);

    assert_eq!(m.read_u16(0), 0x3300);
    assert_eq!(m.read_u16(1), 0x1122);

    assert_eq!(m.read_u32(), 0x11223300);
}

#[test]
fn test_layout_16() {
    let m = B16LevelRegister::new();

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x33);

    assert_eq!(m.read_u8(0), 0x00);
    assert_eq!(m.read_u8(1), 0x33);

    assert_eq!(m.read_u16(), 0x3300);
}

#[test]
fn test_layout_8() {
    let m = B8LevelRegister::new();

    m.write_u8(0x33);

    assert_eq!(m.read_u8(), 0x33);
}

#[test]
fn test_bitfield_32() {
    let m = B32LevelRegister::new();

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x33);
    m.write_u8(2, 0x22);
    m.write_u8(3, 0x11);

    let bitfield = Bitfield::new(9, 4);

    assert_eq!(m.read_bitfield(bitfield), 9);
}

#[test]
fn test_bitfield_16() {
    let m = B16LevelRegister::new();

    m.write_u8(0, 0x00);
    m.write_u8(1, 0x33);

    let bitfield = Bitfield::new(9, 4);

    assert_eq!(m.read_bitfield(bitfield), 9);
}

#[test]
fn test_bitfield_8() {
    let m = B8LevelRegister::new();

    m.write_u8(0x33);

    let bitfield = Bitfield::new(5, 3);

    assert_eq!(m.read_bitfield(bitfield), 1);
}
