//! Edge-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master (CPU/bus)/slave (peripheral) access combination.
//! This will result in data races otherwise!
//! Errors are returned when there are no new values to be read, or when there is an existing value that has not been
//! acknowledged yet.

use super::{
    B16Register_,
    B32Register_,
    B8Register_,
};
use parking_lot::Mutex;
use std::intrinsics::unlikely;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum LatchKind {
    Read,
    Write,
}

pub(crate) struct B32EdgeRegister {
    memory: Mutex<(Option<LatchKind>, B32Register_)>,
}

impl B32EdgeRegister {
    pub(crate) fn new() -> B32EdgeRegister {
        B32EdgeRegister {
            memory: Mutex::new((
                None, 
                B32Register_ { v32: 0 },
            )),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut B32Register_) {
        let data = &mut self.memory.lock();

        if unlikely(data.0.is_some()) {
            return Err(());
        }

        operation(&mut data.1);
        data.0 = Some(latch_kind);
        
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v8[offset as usize];
        })?;
        Ok(value)
    }

    #[allow(dead_code)]
    pub(crate) fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| unsafe {
            r.v8[offset as usize] = value;
        })?;
        Ok(())
    }

    pub(crate) fn read_u16(&self, offset: u32) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v16[offset as usize];
        })?;
        Ok(value)
    }

    pub(crate) fn write_u16(&self, offset: u32, value: u16) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| unsafe {
            r.v16[offset as usize] = value;
        })?;
        Ok(())
    }

    pub(crate) fn read_u32(&self) -> Result<u32, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v32;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u32(&self, value: u32) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            r.v32 = value;
        })?;
        Ok(())
    }

    /// If a latch event is pending, executes an atomic operation to handle it.
    pub(crate) fn acknowledge<F>(&self, operation: F)
    where F: FnOnce(u32, LatchKind) -> u32 {
        let data = &mut self.memory.lock();
        if unlikely(data.0.is_some()) {
            unsafe {
                data.1.v32 = operation(data.1.v32, data.0.unwrap());
            }
            data.0 = None;
        }
    }

    /// Updates the internal value without checking the latch status.
    /// This is used for read-only bits as a part of a whole register.
    pub(crate) fn update<F>(&self, operation: F)
    where F: FnOnce(u32) -> u32 {
        let data = &mut self.memory.lock();
        unsafe {
            data.1.v32 = operation(data.1.v32);
        }
    }
}

unsafe impl Send for B32EdgeRegister {
}

unsafe impl Sync for B32EdgeRegister {
}

pub(crate) struct B16EdgeRegister {
    memory: Mutex<(Option<LatchKind>, B16Register_)>,
}

impl B16EdgeRegister {
    pub(crate) fn new() -> B16EdgeRegister {
        B16EdgeRegister {
            memory: Mutex::new((
                None, 
                B16Register_ { v16: 0 },
            )),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut B16Register_) {
        let data = &mut self.memory.lock();

        if unlikely(data.0.is_some()) {
            return Err(());
        }

        operation(&mut data.1);
        data.0 = Some(latch_kind);

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v8[offset as usize];
        })?;
        Ok(value)
    }

    #[allow(dead_code)]
    pub(crate) fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| unsafe {
            r.v8[offset as usize] = value;
        })?;
        Ok(())
    }

    pub(crate) fn read_u16(&self) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v16;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u16(&self, value: u16) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            r.v16 = value;
        })?;
        Ok(())
    }

    pub(crate) fn acknowledge<F>(&self, operation: F)
    where F: FnOnce(u16, LatchKind) -> u16 {
        let data = &mut self.memory.lock();
        if unlikely(data.0.is_some()) {
            unsafe {
                data.1.v16 = operation(data.1.v16, data.0.unwrap());
            }
            data.0 = None;
        }
    }
    
    #[allow(dead_code)]
    pub(crate) fn update<F>(&self, operation: F)
    where F: FnOnce(u16) -> u16 {
        let data = &mut self.memory.lock();
        unsafe {
            data.1.v16 = operation(data.1.v16);
        }
    }
}

unsafe impl Send for B16EdgeRegister {
}

unsafe impl Sync for B16EdgeRegister {
}

pub(crate) struct B8EdgeRegister {
    memory: Mutex<(Option<LatchKind>, B8Register_)>,
}

impl B8EdgeRegister {
    pub(crate) fn new() -> B8EdgeRegister {
        B8EdgeRegister {
            memory: Mutex::new((
                None, 
                B8Register_ { v8: 0 },
            )),
        }
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()>
    where F: FnOnce(&mut B8Register_) {
        let data = &mut self.memory.lock();

        if unlikely(data.0.is_some()) {
            return Err(());
        }

        operation(&mut data.1);
        data.0 = Some(latch_kind);
        
        Ok(())
    }

    pub(crate) fn read_u8(&self) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe {
            value = r.v8;
        })?;
        Ok(value)
    }

    pub(crate) fn write_u8(&self, value: u8) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| {
            r.v8 = value;
        })?;
        Ok(())
    }

    pub(crate) fn acknowledge<F>(&self, operation: F)
    where F: FnOnce(u8, LatchKind) -> u8 {
        let data = &mut self.memory.lock();
        if unlikely(data.0.is_some()) {
            unsafe {
                data.1.v8 = operation(data.1.v8, data.0.unwrap());
            }
            data.0 = None;
        }
    }
    
    pub(crate) fn update<F>(&self, operation: F)
    where F: FnOnce(u8) -> u8 {
        let data = &mut self.memory.lock();
        unsafe {
            data.1.v8 = operation(data.1.v8);
        }
    }
}

unsafe impl Send for B8EdgeRegister {
}

unsafe impl Sync for B8EdgeRegister {
}
