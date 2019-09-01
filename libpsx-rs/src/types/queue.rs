pub mod debug;

use std::fmt::{Display, UpperHex};
use spsc_ringbuffer::SpscRingbuffer as QueueImpl;
use crate::types::queue::debug::DebugState;

/// SPSC Queue
pub struct Queue<T> 
where
    T: Copy + Default
{
    queue: QueueImpl<T>,
    pub debug_state: Option<DebugState>,
}

impl<T> Queue<T> 
where
    T: Copy + Default + Display + UpperHex
{
    pub fn new(size: usize, debug_state: Option<DebugState>) -> Queue<T> {
        Queue {
            queue: QueueImpl::new(size),
            debug_state: debug_state,
        }
    }

    pub fn read_one(&self) -> Result<T, ()> {
        let result = self.queue.pop().map_err(|_| ());
        
        if result.is_ok() {
            debug::trace_read(self, result.unwrap());
        }

        result
    }

    pub fn write_one(&self, data: T) -> Result<(), ()> {
        let result = self.queue.push(data).map_err(|_| ());

        if result.is_ok() {
            debug::trace_write(self, data);
        }
        
        result
    }

    /// For readers only.
    pub fn read_available(&self) -> usize {
        self.queue.read_available()
    }

    /// For writers only.
    pub fn write_available(&self) -> usize {
        self.queue.write_available()
    }

    /// For readers only.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// For writers only.
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }
}
