use parking_lot::{Mutex, MutexGuard};

// TODO: could probably get this to work with templates, but it's too hard, just use a box...

type AcquireFn<'a, T> = Box<dyn Fn() -> &'a T + 'a>;
type ReleaseFn<'a> = Box<dyn Fn() + 'a>;

pub struct BackendContext<'a, T> {
    context: Mutex<(AcquireFn<'a, T>, ReleaseFn<'a>)>,
}

impl<'a: 'b, 'b, T> BackendContext<'a, T> {
    pub fn new(acquire_context: AcquireFn<'a, T>, release_context: ReleaseFn<'a>) -> BackendContext<'a, T> {
        BackendContext {
            context: Mutex::new((acquire_context, release_context)),
        }
    }

    pub fn guard(&'b self) -> (ContextGuard<'a, 'b, T>, &'a T) {
        let lock = self.context.lock();
        let context = (lock.0)();
        (ContextGuard { guard: lock }, context)
    }
}

pub struct ContextGuard<'a: 'b, 'b, T> {
    guard: MutexGuard<'b, (AcquireFn<'a, T>, ReleaseFn<'a>)>,
}

impl<'a: 'b, 'b, T> Drop for ContextGuard<'a, 'b, T> {
    fn drop(&mut self) { 
        (self.guard.1)();
    }
}
