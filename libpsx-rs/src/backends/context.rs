use parking_lot::{
    Mutex,
    MutexGuard,
};

type AcquireFn<'context, 'closure, T> = Box<dyn (Fn() -> &'context T) + 'closure>;

type ReleaseFn<'closure> = Box<dyn (Fn()) + 'closure>;

pub struct BackendContext<'a: 'b, 'b, T> {
    context: Mutex<(AcquireFn<'a, 'b, T>, ReleaseFn<'b>)>,
}

impl<'a: 'b, 'b, T> BackendContext<'a, 'b, T> {
    pub fn new(acquire_context: AcquireFn<'a, 'b, T>, release_context: ReleaseFn<'a>) -> BackendContext<'a, 'b, T> {
        BackendContext {
            context: Mutex::new((acquire_context, release_context)),
        }
    }
}

impl<'a: 'b, 'b: 'c, 'c, T> BackendContext<'a, 'b, T> {
    pub fn guard(&'c self) -> (ContextGuard<'a, 'b, 'c, T>, &'c T) {
        let lock = self.context.lock();
        let context = (lock.0)();
        (
            ContextGuard {
                guard: lock,
            },
            context,
        )
    }
}

unsafe impl<'a: 'b, 'b, T> Send for BackendContext<'a, 'b, T> {
}

unsafe impl<'a: 'b, 'b, T> Sync for BackendContext<'a, 'b, T> {
}

pub struct ContextGuard<'a: 'b, 'b: 'c, 'c, T> {
    guard: MutexGuard<'c, (AcquireFn<'a, 'b, T>, ReleaseFn<'b>)>,
}

impl<'a: 'b, 'b: 'c, 'c, T> Drop for ContextGuard<'a, 'b, 'c, T> {
    fn drop(&mut self) {
        (self.guard.1)();
    }
}
