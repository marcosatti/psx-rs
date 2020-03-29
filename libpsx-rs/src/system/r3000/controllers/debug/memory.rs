use hashbrown::hash_map::DefaultHashBuilder;
use lazy_static::*;
use lru::LruCache;
use std::cell::UnsafeCell;

struct AccessState(UnsafeCell<LruCache<u32, usize, DefaultHashBuilder>>);

impl AccessState {
    fn new() -> AccessState {
        let hasher = DefaultHashBuilder::default();
        AccessState(UnsafeCell::new(LruCache::with_hasher(32, hasher)))
    }

    fn update(&self, address: u32) -> usize {
        let cache = unsafe { &mut *self.0.get() };
        match cache.get_mut(&address) {
            None => {
                cache.put(address, 1);
                1
            },
            Some(count) => {
                *count += 1;
                *count
            },
        }
    }

    fn clear(&self, address: u32) {
        let cache = unsafe { &mut *self.0.get() };
        cache.pop(&address).unwrap();
    }
}

// Only ever accessed in a single threaded environment.
unsafe impl Sync for AccessState {
}

lazy_static! {
    static ref READS_STATE: AccessState = AccessState::new();
}

lazy_static! {
    static ref WRITES_STATE: AccessState = AccessState::new();
}

pub fn update_state_read(address: u32) -> usize {
    READS_STATE.update(address)
}

pub fn update_state_write(address: u32) -> usize {
    WRITES_STATE.update(address)
}

pub fn clear_state_read(address: u32) {
    READS_STATE.clear(address);
}

pub fn clear_state_write(address: u32) {
    WRITES_STATE.clear(address);
}
