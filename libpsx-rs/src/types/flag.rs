use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct Flag {
    flag: AtomicBool,
}

impl Flag {
    pub(crate) fn new() -> Flag {
        Flag {
            flag: AtomicBool::new(false),
        }
    }

    pub(crate) fn load(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }

    pub(crate) fn store(&self, value: bool) {
        self.flag.store(value, Ordering::Release);
    }

    pub(crate) fn load_barrier(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    pub(crate) fn store_barrier(&self, value: bool) {
        self.flag.store(value, Ordering::SeqCst);
    }
}

impl Clone for Flag {
    fn clone(&self) -> Self {
        Flag {
            flag: AtomicBool::new(self.flag.load(Ordering::Relaxed)),
        }
    }
}
