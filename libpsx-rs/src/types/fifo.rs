#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use spsc_ringbuffer::SpscRingbuffer as QueueImpl;
use std::fmt::{
    Display,
    UpperHex,
};

/// SPSC FIFO
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct Fifo<T>
where T: Copy + Default
{
    fifo: QueueImpl<T>,
}

impl<T> Fifo<T>
where T: Copy + Default + Display + UpperHex
{
    pub(crate) fn new(size: usize) -> Fifo<T> {
        Fifo {
            fifo: QueueImpl::new(size),
        }
    }

    pub(crate) fn read_one(&self) -> Result<T, ()> {
        self.fifo.pop().map_err(|_| ())
    }

    pub(crate) fn write_one(&self, data: T) -> Result<(), ()> {
        self.fifo.push(data).map_err(|_| ())
    }

    pub(crate) fn read_available(&self) -> usize {
        self.fifo.read_available()
    }

    #[allow(dead_code)]
    pub(crate) fn write_available(&self) -> usize {
        self.fifo.write_available()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.fifo.is_empty()
    }

    pub(crate) fn is_full(&self) -> bool {
        self.fifo.is_full()
    }

    pub(crate) fn clear(&self) {
        self.fifo.clear();
    }
}
