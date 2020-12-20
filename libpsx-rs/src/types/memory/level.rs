//! Level-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master/slave access combination (ie: CPU and peripheral).
//! No error handling is required for level-triggered conditions, therefore read/writes always succeed.

use crate::{
    types::bitfield::Bitfield,
    utilities::primitive::*,
};
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::sync::atomic::{
    AtomicU16,
    AtomicU32,
    AtomicU8,
    Ordering,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B32LevelRegister {
    memory: AtomicU32,
}

impl B32LevelRegister {
    pub(crate) fn new() -> B32LevelRegister {
        B32LevelRegister {
            memory: AtomicU32::new(0),
        }
    }

    pub(crate) fn read_u8(&self, offset: u32) -> u8 {
        u32::extract_u8_le(self.memory.load(Ordering::Acquire), offset as usize)
    }

    pub(crate) fn write_u8(&self, offset: u32, value: u8) {
        self.memory.store(u32::insert_u8_le(self.memory.load(Ordering::Acquire), offset as usize, value), Ordering::Release);
    }

    pub(crate) fn read_u16(&self, offset: u32) -> u16 {
        u32::extract_u16_le(self.memory.load(Ordering::Acquire), offset as usize)
    }

    pub(crate) fn write_u16(&self, offset: u32, value: u16) {
        self.memory.store(u32::insert_u16_le(self.memory.load(Ordering::Acquire), offset as usize, value), Ordering::Release);
    }

    pub(crate) fn read_u32(&self) -> u32 {
        self.memory.load(Ordering::Acquire)
    }

    pub(crate) fn write_u32(&self, value: u32) {
        self.memory.store(value, Ordering::Release);
    }

    pub(crate) fn read_bitfield(&self, bitfield: Bitfield) -> u32 {
        bitfield.extract_from(self.read_u32())
    }

    pub(crate) fn write_bitfield(&self, bitfield: Bitfield, value: u32) {
        self.write_u32(bitfield.insert_into(self.read_u32(), value));
    }

    pub(crate) fn write_bitfield_atomic(&self, bitfield: Bitfield, value: u32) {
        loop {
            let register_value = self.memory.load(Ordering::Relaxed);
            let new_register_value = bitfield.insert_into(register_value, value);

            if self.memory.compare_and_swap(register_value, new_register_value, Ordering::AcqRel) == register_value {
                break;
            }
        }
    }
}

unsafe impl Send for B32LevelRegister {
}

unsafe impl Sync for B32LevelRegister {
}

impl Clone for B32LevelRegister {
    fn clone(&self) -> Self {
        B32LevelRegister {
            memory: AtomicU32::new(self.memory.load(Ordering::Relaxed)),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B16LevelRegister {
    memory: AtomicU16,
}

impl B16LevelRegister {
    pub(crate) fn new() -> B16LevelRegister {
        B16LevelRegister {
            memory: AtomicU16::new(0),
        }
    }

    pub(crate) fn read_u8(&self, offset: u32) -> u8 {
        u16::extract_u8_le(self.memory.load(Ordering::Acquire), offset as usize)
    }

    pub(crate) fn write_u8(&self, offset: u32, value: u8) {
        self.memory.store(u16::insert_u8_le(self.memory.load(Ordering::Acquire), offset as usize, value), Ordering::Release);
    }

    pub(crate) fn read_u16(&self) -> u16 {
        self.memory.load(Ordering::Acquire)
    }

    pub(crate) fn write_u16(&self, value: u16) {
        self.memory.store(value, Ordering::Release);
    }

    pub(crate) fn read_bitfield(&self, bitfield: Bitfield) -> u16 {
        bitfield.extract_from(self.read_u16())
    }

    pub(crate) fn write_bitfield(&self, bitfield: Bitfield, value: u16) {
        self.write_u16(bitfield.insert_into(self.read_u16(), value));
    }
}

unsafe impl Send for B16LevelRegister {
}

unsafe impl Sync for B16LevelRegister {
}

impl Clone for B16LevelRegister {
    fn clone(&self) -> Self {
        B16LevelRegister {
            memory: AtomicU16::new(self.memory.load(Ordering::Relaxed)),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B8LevelRegister {
    memory: AtomicU8,
}

impl B8LevelRegister {
    pub(crate) fn new() -> B8LevelRegister {
        B8LevelRegister {
            memory: AtomicU8::new(0),
        }
    }

    pub(crate) fn read_u8(&self) -> u8 {
        self.memory.load(Ordering::Acquire)
    }

    pub(crate) fn write_u8(&self, value: u8) {
        self.memory.store(value, Ordering::Release);
    }

    pub(crate) fn read_bitfield(&self, bitfield: Bitfield) -> u8 {
        bitfield.extract_from(self.read_u8())
    }

    pub(crate) fn write_bitfield(&self, bitfield: Bitfield, value: u8) {
        self.write_u8(bitfield.insert_into(self.read_u8(), value));
    }
}

unsafe impl Send for B8LevelRegister {
}

unsafe impl Sync for B8LevelRegister {
}

impl Clone for B8LevelRegister {
    fn clone(&self) -> Self {
        B8LevelRegister {
            memory: AtomicU8::new(self.memory.load(Ordering::Relaxed)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
