use std::fmt::{Display, UpperHex};
use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::types::fifo::*;

const ENABLE_READ_TRACE: bool = true;
const ENABLE_WRITE_TRACE: bool = true;

pub struct DebugState {
    pub identifier: &'static str,
    trace_reads: bool,
    trace_writes: bool,
    read_count: AtomicUsize,
    write_count: AtomicUsize,
}

impl DebugState {
    pub fn new(identifier: &'static str, trace_reads: bool, trace_writes: bool) -> DebugState {
        DebugState {
            identifier: identifier,
            trace_reads: trace_reads,
            trace_writes: trace_writes,
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
        }
    }
}

pub fn trace_read<T>(fifo: &Fifo<T>, data: T) 
where
    T: Copy + Default + Display + UpperHex
{
    if !ENABLE_READ_TRACE {
        return;
    } 

    let debug_state = match fifo.debug_state {
        None => panic!("Debug state is required to trace FIFO reads"),
        Some(ref d) => d,
    };

    if debug_state.trace_reads {
        let count = debug_state.read_count.fetch_add(1, Ordering::SeqCst);
        trace!("{} ({}): read = 0x{:X}", debug_state.identifier, count, data);
    }
}

pub fn trace_write<T>(fifo: &Fifo<T>, data: T)
where
    T: Copy + Default + Display + UpperHex
{
    if !ENABLE_WRITE_TRACE {
        return;
    } 

    let debug_state = match fifo.debug_state {
        None => panic!("Debug state is required to trace FIFO writes"),
        Some(ref d) => d,
    };

    if debug_state.trace_writes {
        let count = debug_state.write_count.fetch_add(1, Ordering::SeqCst);
        trace!("{} ({}): write = 0x{:X}", debug_state.identifier, count, data);
    }
}
