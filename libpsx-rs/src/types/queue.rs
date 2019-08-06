use spsc_ringbuffer::RingBuffer as QueueImpl;
use log::warn;

/// SPSC Queue
pub struct Queue<T, const N: usize> 
where
    T: Copy + Default
{
    queue: QueueImpl<T, 32>,
}

impl<T, const N: usize> Queue<T, {N}> 
where
    T: Copy + Default
{
    pub fn new() -> Queue<T, {N}> {
        warn!("Queue not fully implemented - requested size of {}, giving 1 less instead", N);

        Queue {
            queue: QueueImpl::new(),
        }
    }

    pub fn read_one(&self) -> Result<T, ()> {
        self.queue.pop().map_err(|_| ())
    }

    pub fn write_one(&self, data: T) -> Result<(), ()> {
        self.queue.push(data).map_err(|_| ())
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
}
