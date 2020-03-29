pub mod debug;

use crate::types::fifo::debug::DebugState;
use spsc_ringbuffer::SpscRingbuffer as QueueImpl;
use std::fmt::{Display, UpperHex};

/// SPSC FIFO
pub struct Fifo<T>
where
    T: Copy + Default,
{
    fifo: QueueImpl<T>,
    pub debug_state: Option<DebugState>,
}

impl<T> Fifo<T>
where
    T: Copy + Default + Display + UpperHex,
{
    pub fn new(size: usize, debug_state: Option<DebugState>) -> Fifo<T> {
        Fifo {
            fifo: QueueImpl::new(size),
            debug_state: debug_state,
        }
    }

    pub fn read_one(&self) -> Result<T, ()> {
        let result = self.fifo.pop().map_err(|_| ());

        if result.is_ok() {
            debug::trace_read(self, result.unwrap());
        }

        result
    }

    pub fn write_one(&self, data: T) -> Result<(), ()> {
        let result = self.fifo.push(data).map_err(|_| ());

        if result.is_ok() {
            debug::trace_write(self, data);
        }

        result
    }

    pub fn read_available(&self) -> usize {
        self.fifo.read_available()
    }

    pub fn write_available(&self) -> usize {
        self.fifo.write_available()
    }

    pub fn is_empty(&self) -> bool {
        self.fifo.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.fifo.is_full()
    }

    pub fn clear(&self) {
        self.fifo.clear();
    }
}
