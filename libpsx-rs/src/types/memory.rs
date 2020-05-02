use crate::types::bitfield::Bitfield;
use std::cell::UnsafeCell;

// TODO: const generics once available.

/// Shared memory (for both registers and blocks of memory).
/// Synchronisation must happen through other means, such as latches.
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

pub struct B32Register {
    memory: UnsafeCell<B32Register_>,
}

impl B32Register {
    pub fn new() -> B32Register {
        B32Register {
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

unsafe impl Send for B32Register {
}

unsafe impl Sync for B32Register {
}

#[repr(C)]
#[derive(Copy, Clone)]
union B16Register_ {
    v16: u16,
    v8: [u8; 2],
}

pub struct B16Register {
    memory: UnsafeCell<B16Register_>,
}

impl B16Register {
    pub fn new() -> B16Register {
        B16Register {
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

unsafe impl Send for B16Register {
}

unsafe impl Sync for B16Register {
}

#[repr(C)]
#[derive(Copy, Clone)]
union B8Register_ {
    v8: u8,
}

pub struct B8Register {
    memory: UnsafeCell<B8Register_>,
}

impl B8Register {
    pub fn new() -> B8Register {
        B8Register {
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

unsafe impl Send for B8Register {
}

unsafe impl Sync for B8Register {
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
