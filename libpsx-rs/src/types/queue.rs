pub mod debug;

use std::fmt::{Display, UpperHex};
use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use spsc_ringbuffer::SpscRingbuffer as QueueImpl;
use crate::types::queue::debug::*;

/// SPSC Queue
pub struct Queue<T> 
where
    T: Copy + Default
{
    queue: QueueImpl<T>,
    identifier: &'static str,
    trace_reads: bool,
    trace_writes: bool,
    read_count: AtomicUsize,
    write_count: AtomicUsize,
}

impl<T> Queue<T> 
where
    T: Copy + Default + Display + UpperHex
{
    pub fn new(size: usize, identifier: &'static str, trace_reads: bool, trace_writes: bool) -> Queue<T> {
        Queue {
            queue: QueueImpl::new(size),
            identifier: identifier,
            trace_reads: trace_reads,
            trace_writes: trace_writes,
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
        }
    }

    pub fn read_one(&self) -> Result<T, ()> {
        let result = self.queue.pop().map_err(|_| ());
        
        if ENABLE_READ_TRACE && self.trace_reads && result.is_ok() {
            let count = self.read_count.load(Ordering::Relaxed);
            trace!("{} ({}): read = 0x{:X}", self.identifier, count, result.unwrap());
            self.read_count.store(count + 1, Ordering::Relaxed);
        }

        result
    }

    pub fn write_one(&self, data: T) -> Result<(), ()> {
        let result = self.queue.push(data).map_err(|_| ());
        
        if ENABLE_WRITE_TRACE && self.trace_writes && result.is_ok() {
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

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }
}
