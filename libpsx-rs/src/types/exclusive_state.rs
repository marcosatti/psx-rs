use parking_lot::Mutex;
use parking_lot::MutexGuard;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct ExclusiveState<T> {
    mutex: Mutex<T>,
}

impl<T> ExclusiveState<T> {
    pub(crate) fn new(value: T) -> ExclusiveState<T> {
        ExclusiveState {
            mutex: Mutex::new(value),
        }
    }

    pub(crate) fn lock(&self) -> MutexGuard<T> {
        self.mutex.lock()
    }

    pub(crate) fn get_mut(&mut self) -> &mut T {
        self.mutex.get_mut()
    }
}

impl<T> Clone for ExclusiveState<T>
where
    T: Clone 
{
    fn clone(&self) -> Self {
        ExclusiveState {
            mutex: Mutex::new(self.mutex.lock().clone()),
        }
    }
}
