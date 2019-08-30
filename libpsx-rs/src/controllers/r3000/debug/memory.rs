use hashbrown::hash_map::DefaultHashBuilder;
use parking_lot::Mutex;
use lazy_static::lazy_static;
use lru::LruCache;

struct MemoryAccessCache(LruCache<u32, usize, DefaultHashBuilder>);

unsafe impl Send for MemoryAccessCache {}

fn make_cache() -> Mutex<MemoryAccessCache> {
    let hasher = DefaultHashBuilder::default();
    Mutex::new(MemoryAccessCache(LruCache::with_hasher(32, hasher)))
}

lazy_static! {
    static ref MEMORY_READS_CACHE: Mutex<MemoryAccessCache> = make_cache();
}

lazy_static! {
    static ref MEMORY_WRITES_CACHE: Mutex<MemoryAccessCache> = make_cache();
}

fn update_memory_access(cache: &mut MemoryAccessCache, address: u32) -> usize {
    match cache.0.get_mut(&address) {
        None => {
            cache.0.put(address, 1);
            1
        },
        Some(count) => {
            *count += 1;
            *count
        }
    }
}

fn remove_memory_address(cache: &mut MemoryAccessCache, address: u32) {
    cache.0.pop(&address).unwrap();
}

pub fn track_memory_read(address: u32) -> usize {
    let cache = &mut MEMORY_READS_CACHE.lock();
    update_memory_access(cache, address)
}

pub fn track_memory_write(address: u32) -> usize {
    let cache = &mut MEMORY_WRITES_CACHE.lock();
    update_memory_access(cache, address)
}

pub fn track_memory_read_clear(address: u32) {
    let cache = &mut MEMORY_READS_CACHE.lock();
    remove_memory_address(cache, address);
}

pub fn track_memory_write_clear(address: u32) {
    let cache = &mut MEMORY_WRITES_CACHE.lock();
    remove_memory_address(cache, address);
}
