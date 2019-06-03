use std::collections::VecDeque;
use std::ops::Index;
use spin::Mutex;

/// SPSC Queue
/// Locking is needed for read only operations due to a possible mutable operation occuring in another thread.
/// TODO: explore https://crates.io/crates/heapless
pub struct Queue<T> 
where
    T: Copy + Default
{
    pub queue: VecDeque<T>,
    pub mutex: Mutex<()>,
}

impl<T> Queue<T> 
where
    T: Copy + Default
{
    pub fn new() -> Queue<T> {
        Queue {
            queue: VecDeque::new(),
            mutex: Mutex::new(()),
        }
    }

    pub fn len(&self) -> usize {
        let _lock = self.mutex.lock();
        self.queue.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek_front(&self) -> Option<T> {
        let _lock = self.mutex.lock();
        self.queue.front().map(|v| *v)
    }

    pub fn read_one(&mut self) -> Option<T> {
        if self.empty() { return None; }
        let _lock = self.mutex.lock();
        self.queue.pop_front()
    }

    pub fn write_one(&mut self, data: T) {
        let _lock = self.mutex.lock();
        self.queue.push_back(data);
    } 

    // For when const generics support lands...

    // pub fn read<const N: usize>(&mut self) -> Option<[T; N]> {
    //     if self.len() < N { return None; }
    //     let _lock = self.mutex.lock();
    //     self.queue.drain(0..N).collect()
    // }

    pub fn read(&mut self, n: usize) -> Option<Vec<T>> {
        if self.len() < n { return None; }
        let _lock = self.mutex.lock();
        Some(self.queue.drain(0..n).collect())
    }

    pub fn write<I: IntoIterator<Item=T>>(&mut self, data: I) {
        let _lock = self.mutex.lock();
        self.queue.extend(data);
    }

    pub fn clear(&mut self) {
        let _lock = self.mutex.lock();
        self.queue.clear();
    }
}

impl<T> Index<usize> for Queue<T>
where
    T: Copy + Default
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let _lock = self.mutex.lock();
        &self.queue[index]
    }
}
