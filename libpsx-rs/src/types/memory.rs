mod edge;
mod level;

pub(crate) use edge::*;
pub(crate) use level::*;
use std::cell::UnsafeCell;

// TODO: const generics once available.

/// Shared memory
/// No synchronisation is provided, it must be done through other means.
pub(crate) struct B8Memory {
    memory: UnsafeCell<Vec<u8>>,
}

impl B8Memory {
    pub(crate) fn new(size: usize) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(vec![0; size]),
        }
    }

    pub(crate) fn new_initialized(size: usize, value: u8) -> B8Memory {
        B8Memory {
            memory: UnsafeCell::new(vec![value; size]),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn read_raw(&self, byte_offset: u32) -> &[u8] {
        unsafe { &(*self.memory.get())[byte_offset as usize..] }
    }

    pub(crate) fn write_raw(&self, byte_offset: u32, data: &[u8]) {
        unsafe {
            (*self.memory.get())[byte_offset as usize..(byte_offset as usize) + data.len()].copy_from_slice(data);
        }
    }

    pub(crate) fn read_u8(&self, byte_offset: u32) -> u8 {
        unsafe { (*self.memory.get())[byte_offset as usize] }
    }

    pub(crate) fn write_u8(&self, byte_offset: u32, value: u8) {
        unsafe {
            (*self.memory.get())[byte_offset as usize] = value;
        }
    }

    pub(crate) fn read_u16(&self, byte_offset: u32) -> u16 {
        assert_eq!(byte_offset % 2, 0);
        unsafe { *((&(*self.memory.get())[byte_offset as usize] as *const u8) as *const u16) }
    }

    pub(crate) fn write_u16(&self, byte_offset: u32, value: u16) {
        assert_eq!(byte_offset % 2, 0);
        unsafe {
            *((&mut (*self.memory.get())[byte_offset as usize] as *mut u8) as *mut u16) = value;
        }
    }

    pub(crate) fn read_u32(&self, byte_offset: u32) -> u32 {
        assert_eq!(byte_offset % 4, 0);
        unsafe { *((&(*self.memory.get())[byte_offset as usize] as *const u8) as *const u32) }
    }

    pub(crate) fn write_u32(&self, byte_offset: u32, value: u32) {
        assert_eq!(byte_offset % 4, 0);
        unsafe {
            *((&mut (*self.memory.get())[byte_offset as usize] as *mut u8) as *mut u32) = value;
        }
    }
}

unsafe impl Send for B8Memory {
}

unsafe impl Sync for B8Memory {
}

#[cfg(feature = "serialization")]
mod serialization {
    use super::*;
    use serde::{
        Deserialize,
        Deserializer,
        Serialize,
        Serializer,
    };

    impl Serialize for B8Memory {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            let buffer = unsafe { &*self.memory.get() };
            <Vec<u8> as Serialize>::serialize(buffer, serializer)
        }
    }

    impl<'de> Deserialize<'de> for B8Memory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            let buffer = <Vec<u8> as Deserialize>::deserialize(deserializer)?;
            Ok(B8Memory {
                memory: UnsafeCell::new(buffer),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
