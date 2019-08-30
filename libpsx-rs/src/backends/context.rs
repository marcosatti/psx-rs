use parking_lot::{Mutex, MutexGuard};

pub struct BackendContext<'a, T> {
    acquire_release: Mutex<(&'a (dyn Fn() -> &'a T), &'a dyn Fn())>,
}

impl<'a: 'b, 'b, T> BackendContext<'a, T> {
    pub fn new(acquire_context: &'a (dyn Fn() -> &'a T), release_context: &'a dyn Fn()) -> BackendContext<'a, T> {
        BackendContext {
            acquire_release: Mutex::new((acquire_context, release_context)),
        }
    }

    pub fn guard(&'b self) -> (ContextGuard<'a, 'b, T>, &'a T) {
        let lock = self.acquire_release.lock();
        let context = (lock.0)();
        (ContextGuard { guard: lock }, context)
    }
}

pub struct ContextGuard<'a: 'b, 'b, T> {
    guard: MutexGuard<'b, (&'a (dyn Fn() -> &'a T), &'a dyn Fn())>,
}

impl<'a: 'b, 'b, T> Drop for ContextGuard<'a, 'b, T> {
    fn drop(&mut self) { 
        (self.guard.1)();
    }
}
