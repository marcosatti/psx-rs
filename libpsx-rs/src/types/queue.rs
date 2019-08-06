use std::sync::Mutex;
use fixed_vec_deque::FixedVecDeque as QueueImpl;
use log::warn;

/// SPSC Queue
pub struct Queue<T, const N: usize> 
where
    T: Copy + Default
{
    mutex: Mutex<()>,
    queue: QueueImpl<[T; 32]>,
}

impl<T, const N: usize> Queue<T, {N}> 
where
    T: Copy + Default
{
    pub fn new() -> Queue<T, {N}> {
        warn!("Queue not fully implemented - requested size of {}, giving 32 instead", N);

        Queue {
            mutex: Mutex::new(()),
            queue: QueueImpl::new(),
        }
    }

    pub fn read_one(&mut self) -> Result<T, ()> {
        if self.queue.is_empty() {
            return Err(());
        }

        let _lock = self.mutex.lock().unwrap();
        Ok(*self.queue.pop_front().unwrap())
    }

    pub fn write_one(&mut self, data: T) -> Result<(), ()> {
        if self.queue.is_full() {
            return Err(());
        }

        let _lock = self.mutex.lock().unwrap();
        *self.queue.push_back() = data;
        Ok(())
    } 
    
    pub fn read<const N2: usize>(&mut self) -> Result<[T; N2], ()> {
        if self.queue.len() < N2 {
            return Err(());
        }

        let data: [T; N2] = [T::default(); N2];

        let _lock = self.mutex.lock().unwrap();
        for i in 0..N2 {
            data[i] = *self.queue.pop_front().unwrap()
        }

        Ok(data)
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
