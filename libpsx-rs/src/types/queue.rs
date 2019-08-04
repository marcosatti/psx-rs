use heapless::spsc::Queue as QueueImpl;
use heapless::consts::*;
use log::warn;

/// SPSC Queue
pub struct Queue<T, const N: usize> 
where
    T: Copy + Default
{
    pub queue: QueueImpl<T, U256>,
}

impl<T, const N: usize> Queue<T, {N}> 
where
    T: Copy + Default
{
    pub fn new() -> Queue<T, {N}> {
        warn!("Queue not fully implemented - requested size of {}, giving 256 instead", N);

        Queue {
            queue: QueueImpl::u8(),
        }
    }

    pub fn read_one(&mut self) -> Result<T, ()> {
        self.queue.dequeue().ok_or(())
    }

    pub fn write_one(&mut self, data: T) -> Result<(), ()> {
        self.queue.enqueue(data).map_err(|_| ())
    } 
}
