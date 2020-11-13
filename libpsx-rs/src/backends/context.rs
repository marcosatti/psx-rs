use parking_lot::{
    Mutex,
    MutexGuard,
};

pub trait Acquire<Ctx: Copy> = Fn() -> Ctx;

pub trait Release = Fn() -> ();

type InternalContext<'a, Ctx> = (&'a (dyn Acquire<Ctx> + 'a), &'a (dyn Release + 'a));

pub struct BackendContext<'a, Ctx: Copy> {
    context: Mutex<InternalContext<'a, Ctx>>,
}

impl<'a, Ctx: Copy> BackendContext<'a, Ctx> {
    pub fn new(acquire_context: &'a (dyn Acquire<Ctx> + 'a), release_context: &'a (dyn Release + 'a)) -> BackendContext<'a, Ctx> {
        BackendContext {
            context: Mutex::new((acquire_context, release_context)),
        }
    }
}

impl<'a: 'b, 'b, Ctx: Copy> BackendContext<'a, Ctx> {
    pub fn guard(&'b self) -> (ContextGuard<'a, 'b, Ctx>, Ctx) {
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

unsafe impl<'a, Ctx: Copy> Send for BackendContext<'a, Ctx> {
}

unsafe impl<'a, Ctx: Copy> Sync for BackendContext<'a, Ctx> {
}

pub struct ContextGuard<'a: 'b, 'b, Ctx> {
    guard: MutexGuard<'b, InternalContext<'a, Ctx>>,
}

impl<'a: 'b, 'b, Ctx> Drop for ContextGuard<'a, 'b, Ctx> {
    fn drop(&mut self) {
        (self.guard.1)();
    }
}
