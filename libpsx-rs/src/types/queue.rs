use std::fmt::{Display, UpperHex};
use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use spsc_ringbuffer::SpscRingbuffer as QueueImpl;
use crate::debug::ENABLE_FIFO_TRACE;

/// SPSC Queue
pub struct Queue<T> 
where
    T: Copy + Default
{
    queue: QueueImpl<T>,
    identifier: String,
    trace: bool,
    read_count: AtomicUsize,
    write_count: AtomicUsize,
}

impl<T> Queue<T> 
where
    T: Copy + Default + Display + UpperHex
{
    pub fn new(size: usize, identifier: &str, trace: bool) -> Queue<T> {
        Queue {
            queue: QueueImpl::new(size),
            identifier: identifier.to_owned(),
            trace: trace,
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
        }
    }

    pub fn read_one(&self) -> Result<T, ()> {
        let result = self.queue.pop().map_err(|_| ());
        
        if ENABLE_FIFO_TRACE && self.trace && result.is_ok() {
            let count = self.read_count.load(Ordering::Relaxed);
            trace!("{} ({}): read = 0x{:X}", self.identifier, count, result.unwrap());
            self.read_count.store(count + 1, Ordering::Relaxed);
        }

        result
    }

    pub fn write_one(&self, data: T) -> Result<(), ()> {
        let result = self.queue.push(data).map_err(|_| ());
        
        if ENABLE_FIFO_TRACE && self.trace && result.is_ok() {
            let count = self.write_count.load(Ordering::Relaxed);
            trace!("{} ({}): write = 0x{:X}", self.identifier, count, data);
            self.write_count.store(count + 1, Ordering::Relaxed);
        }
        
        result
    } 
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    pub fn is_full(&self) -> bool {
        self.queue.is_full()
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}
