//! Edge-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master (CPU/bus)/slave (peripheral) access combination.
//! This will result in data races otherwise!
//! Errors are returned when there are no new values to be read, or when there is an existing value that has not been
//! acknowledged yet.
//! The read/write function are intended to be used from the master side, and acknowledge/update from the slave side.

use crate::utilities::primitive::*;
use parking_lot::Mutex;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum LatchKind {
    Read,
    Write,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B32EdgeRegister {
    dirty: AtomicBool,
    memory: Mutex<(LatchKind, u32)>,
}

impl B32EdgeRegister {
    pub(crate) fn new() -> B32EdgeRegister {
        B32EdgeRegister {
            dirty: AtomicBool::new(false),
            memory: Mutex::new((LatchKind::Read, 0)),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut u32) {
        if self.dirty.load(Ordering::Acquire) {
            return Err(());
        }

        {
            let data = &mut self.memory.lock();
            data.0 = latch_kind;
            operation(&mut data.1);
        }
        self.dirty.store(true, Ordering::Release);

        Ok(())
    }

    pub(crate) fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| value = u32::extract_u8_le(*r, offset as usize))?;
        Ok(value)
    }

    pub(crate) fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = u32::insert_u8_le(*r, offset as usize, value);
        })?;
        Ok(())
    }

    pub(crate) fn read_u16(&self, offset: u32) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| value = u32::extract_u16_le(*r, offset as usize))?;
        Ok(value)
    }

    pub(crate) fn write_u16(&self, offset: u32, value: u16) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = u32::insert_u16_le(*r, offset as usize, value);
        })?;
        Ok(())
    }

    pub(crate) fn read_u32(&self) -> Result<u32, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| {
            value = *r;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u32(&self, value: u32) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = value;
        })?;
        Ok(())
    }

    /// If a latch event is pending, executes an atomic operation to handle it.
    pub(crate) fn acknowledge<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u32, LatchKind) -> Result<u32, E> {
        if self.dirty.load(Ordering::Acquire) {
            let data = &mut self.memory.lock();
            data.1 = operation(data.1, data.0)?;
            self.dirty.store(false, Ordering::Release);
        }

        Ok(())
    }

    /// Updates the internal value without checking the latch status.
    /// This is used for read-only bits as a part of a whole register.
    pub(crate) fn update<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u32) -> Result<u32, E> {
        let data = &mut self.memory.lock();
        data.1 = operation(data.1)?;
        Ok(())
    }
}

unsafe impl Send for B32EdgeRegister {
}

unsafe impl Sync for B32EdgeRegister {
}

impl Clone for B32EdgeRegister {
    fn clone(&self) -> Self {
        B32EdgeRegister {
            dirty: AtomicBool::new(self.dirty.load(Ordering::Relaxed)),
            memory: Mutex::new(self.memory.lock().clone()),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B16EdgeRegister {
    dirty: AtomicBool,
    memory: Mutex<(LatchKind, u16)>,
}

impl B16EdgeRegister {
    pub(crate) fn new() -> B16EdgeRegister {
        B16EdgeRegister {
            dirty: AtomicBool::new(false),
            memory: Mutex::new((LatchKind::Read, 0)),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut u16) {
        if self.dirty.load(Ordering::Acquire) {
            return Err(());
        }

        {
            let data = &mut self.memory.lock();
            data.0 = latch_kind;
            operation(&mut data.1);
        }
        self.dirty.store(true, Ordering::Release);

        Ok(())
    }

    pub(crate) fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| value = u16::extract_u8_le(*r, offset as usize))?;
        Ok(value)
    }

    pub(crate) fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = u16::insert_u8_le(*r, offset as usize, value);
        })?;
        Ok(())
    }

    pub(crate) fn read_u16(&self) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| {
            value = *r;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u16(&self, value: u16) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = value;
        })?;
        Ok(())
    }

    pub(crate) fn acknowledge<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u16, LatchKind) -> Result<u16, E> {
        if self.dirty.load(Ordering::Acquire) {
            let data = &mut self.memory.lock();
            data.1 = operation(data.1, data.0)?;
            self.dirty.store(false, Ordering::Release);
        }

        Ok(())
    }

    pub(crate) fn update<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u16) -> Result<u16, E> {
        let data = &mut self.memory.lock();
        data.1 = operation(data.1)?;
        Ok(())
    }
}

unsafe impl Send for B16EdgeRegister {
}

unsafe impl Sync for B16EdgeRegister {
}

impl Clone for B16EdgeRegister {
    fn clone(&self) -> Self {
        B16EdgeRegister {
            dirty: AtomicBool::new(self.dirty.load(Ordering::Relaxed)),
            memory: Mutex::new(self.memory.lock().clone()),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct B8EdgeRegister {
    dirty: AtomicBool,
    memory: Mutex<(LatchKind, u8)>,
}

impl B8EdgeRegister {
    pub(crate) fn new() -> B8EdgeRegister {
        B8EdgeRegister::with_value(0)
    }

    pub(crate) fn with_value(value: u8) -> B8EdgeRegister {
        B8EdgeRegister {
            dirty: AtomicBool::new(false),
            memory: Mutex::new((LatchKind::Read, value)),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut u8) {
        if self.dirty.load(Ordering::Acquire) {
            return Err(());
        }

        {
            let data = &mut self.memory.lock();
            data.0 = latch_kind;
            operation(&mut data.1);
        }
        self.dirty.store(true, Ordering::Release);

        Ok(())
    }

    pub(crate) fn read_u8(&self) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| {
            value = *r;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u8(&self, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            *r = value;
        })?;
        Ok(())
    }

    pub(crate) fn acknowledge<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u8, LatchKind) -> Result<u8, E> {
        if self.dirty.load(Ordering::Acquire) {
            let data = &mut self.memory.lock();
            data.1 = operation(data.1, data.0)?;
            self.dirty.store(false, Ordering::Release);
        }

        Ok(())
    }

    pub(crate) fn update<F, E>(&self, operation: F) -> Result<(), E>
    where F: FnOnce(u8) -> Result<u8, E> {
        let data = &mut self.memory.lock();
        data.1 = operation(data.1)?;
        Ok(())
    }
}

unsafe impl Send for B8EdgeRegister {
}

unsafe impl Sync for B8EdgeRegister {
}

impl Clone for B8EdgeRegister {
    fn clone(&self) -> Self {
        B8EdgeRegister {
            dirty: AtomicBool::new(self.dirty.load(Ordering::Relaxed)),
            memory: Mutex::new(self.memory.lock().clone()),
        }
    }
}
