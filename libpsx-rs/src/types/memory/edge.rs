//! Edge-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master (CPU/bus)/slave (peripheral) access combination.
//! Errors are returned when there are no new values to be read, or when there is an existing value that has not been acknowledged yet.

use parking_lot::Mutex;
use super::{B32Register_};

struct EdgeRegister<RegisterTy> {
    memory: RegisterTy,
    latch_status: Option<LatchKind>,
}

#[derive(Debug, Copy, Clone)]
pub enum LatchKind {
    Read,
    Write,
}

pub struct B32EdgeRegister(Mutex<EdgeRegister<B32Register_>>);

impl B32EdgeRegister {
    pub fn new() -> B32EdgeRegister {
        B32EdgeRegister(
            Mutex::new(EdgeRegister {
                memory: B32Register_ { v32: 0 }, 
                latch_status: None,
            }),
        )
    }

    fn try_op<F>(&self, latch_kind: LatchKind, operation: F) -> Result<(), ()> 
    where 
        F: FnOnce(&mut B32Register_),
    {
        let data = &mut self.0.lock();

        if data.latch_status.is_some() {
            return Err(());
        }
        
        data.latch_status = Some(latch_kind);
        operation(&mut data.memory);
        Ok(())
    }

    pub fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe { value = r.v8[offset as usize]; })?;
        Ok(value)
    }

    pub fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op( LatchKind::Write, |r| unsafe { r.v8[offset as usize] = value; })?;
        Ok(())
    }

    pub fn read_u16(&self, offset: u32) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe { value = r.v16[offset as usize]; })?;
        Ok(value)
    }

    pub fn write_u16(&self, offset: u32, value: u16) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| unsafe { r.v16[offset as usize] = value; })?;
        Ok(())
    }

    pub fn read_u32(&self) -> Result<u32, ()> {
        let mut value = 0;
        self.try_op(LatchKind::Read, |r| unsafe { value = r.v32; })?;
        Ok(value)
    }

    pub fn write_u32(&self, value: u32) -> Result<(), ()> {
        self.try_op(LatchKind::Write, |r| { r.v32 = value; })?;
        Ok(())
    }

    /// If a latch event is pending, executes an atomic operation to handle it.
    pub fn acknowledge<F>(&self, operation: F) 
    where
        F: FnOnce(u32, LatchKind) -> u32, 
    {
        let data = &mut self.0.lock();
        if let Some(latch_kind) = data.latch_status {
            data.memory.v32 = operation(unsafe { data.memory.v32 }, latch_kind);
            data.latch_status = None;
        }
    }

    /// Updates the internal value without checking the latch status.
    /// This is used for read-only bits as a part of a whole register.
    pub fn update<F>(&self, operation: F)
    where
        F: FnOnce(u32) -> u32, 
    {
        let data = &mut self.0.lock();
        data.memory.v32 = operation(unsafe { data.memory.v32 });
    }
}

unsafe impl Send for B32EdgeRegister {
}

unsafe impl Sync for B32EdgeRegister {
}
