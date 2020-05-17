use hashbrown::hash_map::DefaultHashBuilder;
use lazy_static::*;
use lru::LruCache;
use parking_lot::Mutex;

struct AccessState(Mutex<LruCache<u32, usize, DefaultHashBuilder>>);

impl AccessState {
    fn new() -> AccessState {
        AccessState(Mutex::new(LruCache::with_hasher(32, DefaultHashBuilder::default())))
    }

    fn update(&self, address: u32) -> usize {
        let mut cache = self.0.lock();
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
        self.0.lock().pop(&address).unwrap();
    }
}

unsafe impl Sync for AccessState {
}

lazy_static! {
    static ref READS_STATE: AccessState = AccessState::new();
}

lazy_static! {
    static ref WRITES_STATE: AccessState = AccessState::new();
}

pub(crate) fn update_state_read(address: u32) -> usize {
    READS_STATE.update(address)
}

pub(crate) fn update_state_write(address: u32) -> usize {
    WRITES_STATE.update(address)
}

pub(crate) fn clear_state_read(address: u32) {
    READS_STATE.clear(address);
}

pub(crate) fn clear_state_write(address: u32) {
    WRITES_STATE.clear(address);
}
