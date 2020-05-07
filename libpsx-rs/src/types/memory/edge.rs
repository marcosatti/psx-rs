//! Edge-triggered shared registers, for use in peripheral I/O memory mapped scenarios.
//! Assumes only a single master/slave access combination (ie: CPU and peripheral). 
//! Errors are returned when there are no new values to be read, or when there is an existing value that has not been acknowledged yet.

use parking_lot::Mutex;
use super::{B32Register_, B16Register_, B8Register_};
use std::mem::size_of;

trait LatchAccessor {
    fn at<'a>(&'a mut self, index: usize) -> &'a mut bool; 
}

impl LatchAccessor for &mut [bool; size_of::<u8>()] {
    fn at<'a>(&'a mut self, index: usize) -> &'a mut bool { 
        &mut self[index] 
    }
}

impl LatchAccessor for &mut [bool; size_of::<u16>()] {
    fn at<'a>(&'a mut self, index: usize) -> &'a mut bool { 
        &mut self[index] 
    }
}

impl LatchAccessor for &mut [bool; size_of::<u32>()] {
    fn at<'a>(&'a mut self, index: usize) -> &'a mut bool { 
        &mut self[index] 
    }
}

fn update_latches<L: LatchAccessor, Primitive>(offset: u32, writing: bool, mut latches: L) -> Result<(), ()> {
    // Work out byte latches required.
    let start = offset as usize * size_of::<Primitive>();
    let end = (offset as usize + 1) * size_of::<Primitive>();
    
    // Check that all byte latches are either off or on, depending on if reading or writing.
    // If reading, we want all byte latches to be set.
    // If writing, we want all byte latches to be unset.
    let mut all_latched = true;
    for i in start..end {
        all_latched &= writing ^ *latches.at(i);
    }

    if all_latched {
        // Conditions are ok so update the latches and return ok status.
        for i in start..end {
            *latches.at(i) = writing;
        }

        Ok(())
    } else {
        // At least one of the byte latches was in the wrong condition so bail out.
        Err(())
    }
}

pub struct B32EdgeRegister {
    memory: Mutex<(B32Register_, [bool; size_of::<u32>()])>,
}

impl B32EdgeRegister {
    pub fn new() -> B32EdgeRegister {
        B32EdgeRegister {
            memory: Mutex::new((
                B32Register_ { v32: 0 }, 
                [false; size_of::<u32>()],
            )),
        }
    }

    fn try_op<F, Primitive>(&self, offset: u32, writing: bool, op: F) -> Result<(), ()> 
    where 
        F: FnOnce(&mut B32Register_),
    {
        let data = &mut self.memory.lock();
        update_latches::<_, Primitive>(offset, writing, &mut data.1)?;
        op(&mut data.0);
        Ok(())
    }

    pub fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op::<_, u8>(offset, false, |r| unsafe { value = r.v8[offset as usize]; })?;
        Ok(value)
    }

    pub fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op::<_, u8>(offset, true, |r| unsafe { r.v8[offset as usize] = value; })?;
        Ok(())
    }

    pub fn read_u16(&self, offset: u32) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op::<_, u16>(offset, false, |r| unsafe { value = r.v16[offset as usize]; })?;
        Ok(value)
    }

    pub fn write_u16(&self, offset: u32, value: u16) -> Result<(), ()> {
        self.try_op::<_, u16>(offset, true, |r| unsafe { r.v16[offset as usize] = value; })?;
        Ok(())
    }

    pub fn read_u32(&self) -> Result<u32, ()> {
        let mut value = 0;
        self.try_op::<_, u32>(0, false, |r| unsafe { value = r.v32; })?;
        Ok(value)
    }

    pub fn write_u32(&self, value: u32) -> Result<(), ()> {
        self.try_op::<_, u32>(0, true, |r| { r.v32 = value; })?;
        Ok(())
    }

    pub fn acknowledged(&self) -> bool {
        !(self.memory.lock().1.iter().fold(false, |acc, elm| acc | *elm))
    }

    pub fn clear(&self) {
        self.memory.lock().1.iter_mut().for_each(|elm| { *elm = false; });
    }
}

unsafe impl Send for B32EdgeRegister {
}

unsafe impl Sync for B32EdgeRegister {
}

pub struct B16EdgeRegister {
    memory: Mutex<(B16Register_, [bool; size_of::<u16>()])>,
}

impl B16EdgeRegister {
    pub fn new() -> B16EdgeRegister {
        B16EdgeRegister {
            memory: Mutex::new((
                B16Register_ { v16: 0 }, 
                [false; size_of::<u16>()],
            )),
        }
    }

    fn try_op<F, Primitive>(&self, offset: u32, writing: bool, op: F) -> Result<(), ()> 
    where 
        F: FnOnce(&mut B16Register_),
    {
        let data = &mut self.memory.lock();
        update_latches::<_, Primitive>(offset, writing, &mut data.1)?;
        op(&mut data.0);
        Ok(())
    }

    pub fn read_u8(&self, offset: u32) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op::<_, u8>(offset, false, |r| unsafe { value = r.v8[offset as usize]; })?;
        Ok(value)
    }

    pub fn write_u8(&self, offset: u32, value: u8) -> Result<(), ()> {
        self.try_op::<_, u8>(offset, true, |r| unsafe { r.v8[offset as usize] = value; })?;
        Ok(())
    }

    pub fn read_u16(&self) -> Result<u16, ()> {
        let mut value = 0;
        self.try_op::<_, u16>(0, false, |r| unsafe { value = r.v16; })?;
        Ok(value)
    }

    pub fn write_u16(&self, value: u16) -> Result<(), ()> {
        self.try_op::<_, u16>(0, true, |r| { r.v16 = value; })?;
        Ok(())
    }

    pub fn acknowledged(&self) -> bool {
        !(self.memory.lock().1.iter().fold(false, |acc, elm| acc | *elm))
    }

    pub fn clear(&self) {
        self.memory.lock().1.iter_mut().for_each(|elm| { *elm = false; });
    }
}

unsafe impl Send for B16EdgeRegister {
}

unsafe impl Sync for B16EdgeRegister {
}

pub struct B8EdgeRegister {
    memory: Mutex<(B8Register_, [bool; size_of::<u8>()])>,
}

impl B8EdgeRegister {
    pub fn new() -> B8EdgeRegister {
        B8EdgeRegister {
            memory: Mutex::new((
                B8Register_ { v8: 0 }, 
                [false; size_of::<u8>()],
            )),
        }
    }

    fn try_op<F, Primitive>(&self, offset: u32, writing: bool, op: F) -> Result<(), ()> 
    where 
        F: FnOnce(&mut B8Register_),
    {
        let data = &mut self.memory.lock();
        update_latches::<_, Primitive>(offset, writing, &mut data.1)?;
        op(&mut data.0);
        Ok(())
    }

    pub fn read_u8(&self) -> Result<u8, ()> {
        let mut value = 0;
        self.try_op::<_, u8>(0, false, |r| unsafe { value = r.v8; })?;
        Ok(value)
    }

    pub fn write_u8(&self, value: u8) -> Result<(), ()> {
        self.try_op::<_, u8>(0, true, |r| { r.v8 = value; })?;
        Ok(())
    }

    pub fn acknowledged(&self) -> bool {
        !(self.memory.lock().1.iter().fold(false, |acc, elm| acc | *elm))
    }
    
    pub fn clear(&self) {
        self.memory.lock().1.iter_mut().for_each(|elm| { *elm = false; });
    }
}

unsafe impl Send for B8EdgeRegister {
}

unsafe impl Sync for B8EdgeRegister {
}

#[test]
fn test_acknowledge_32() {
    let r = B32EdgeRegister::new();
    
    assert_eq!(r.read_u32(), Err(()));
    assert_eq!(r.write_u32(0x33221100), Ok(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.write_u32(0x11223344), Err(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.read_u32(), Ok(0x33221100));
    assert_eq!(r.read_u32(), Err(()));
    assert_eq!(r.acknowledged(), true);

    assert_eq!(r.write_u16(0, 0x5566), Ok(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.read_u32(), Err(()));
    assert_eq!(r.write_u16(1, 0x7788), Ok(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.read_u32(), Ok(0x77885566));
    assert_eq!(r.acknowledged(), true);
    assert_eq!(r.read_u16(0), Err(()));
    assert_eq!(r.read_u16(1), Err(()));
    assert_eq!(r.read_u32(), Err(()));
    assert_eq!(r.acknowledged(), true);
    
    assert_eq!(r.write_u8(0, 0x12), Ok(()));
    assert_eq!(r.read_u8(1), Err(()));
    assert_eq!(r.read_u8(2), Err(()));
    assert_eq!(r.read_u8(3), Err(()));
    assert_eq!(r.write_u32(0x11223344), Err(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.write_u16(0, 0x7788), Err(()));
    assert_eq!(r.read_u8(0), Ok(0x12));
    assert_eq!(r.acknowledged(), true);
    assert_eq!(r.write_u16(1, 0x3456), Ok(()));
    assert_eq!(r.read_u8(0), Err(()));
    assert_eq!(r.read_u8(1), Err(()));
    assert_eq!(r.read_u8(2), Ok(0x56));
    assert_eq!(r.read_u8(3), Ok(0x34));
    assert_eq!(r.acknowledged(), true);
}

#[test]
fn test_acknowledge_16() {
    let r = B16EdgeRegister::new();

    assert_eq!(r.acknowledged(), true);
    assert_eq!(r.write_u16(0x5566), Ok(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.read_u16(), Ok(0x5566));
    assert_eq!(r.acknowledged(), true);
}

#[test]
fn test_acknowledge_8() {
    let r = B8EdgeRegister::new();

    assert_eq!(r.acknowledged(), true);
    assert_eq!(r.write_u8(0x55), Ok(()));
    assert_eq!(r.acknowledged(), false);
    assert_eq!(r.read_u8(), Ok(0x55));
    assert_eq!(r.acknowledged(), true);
}
