use parking_lot::{
    Mutex,
    MutexGuard,
};

pub trait Acquire<Ctx: Copy> = Fn() -> Ctx;
pub trait Release = Fn() -> ();

type InternalContext<'ctx, Ctx> = (&'ctx dyn Acquire<Ctx>, &'ctx dyn Release);

pub struct BackendContext<'ctx, Ctx: Copy> {
    context: Mutex<InternalContext<'ctx, Ctx>>,
}

impl<'ctx, Ctx: Copy> BackendContext<'ctx, Ctx> {
    pub fn new(acquire_context: &'ctx dyn Acquire<Ctx>, release_context: &'ctx dyn Release) -> BackendContext<'ctx, Ctx> {
        BackendContext {
            context: Mutex::new((acquire_context, release_context)),
        }
    }
}

impl<'ctx: 'gd, 'gd, Ctx: Copy> BackendContext<'ctx, Ctx> {
    pub fn guard(&'gd self) -> (ContextGuard<'ctx, 'gd, Ctx>, Ctx) {
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

unsafe impl<'ctx, Ctx: Copy> Send for BackendContext<'ctx, Ctx> {
}

unsafe impl<'ctx, Ctx: Copy> Sync for BackendContext<'ctx, Ctx> {
}

pub struct ContextGuard<'ctx: 'gd, 'gd, Ctx> {
    guard: MutexGuard<'gd, InternalContext<'ctx, Ctx>>,
}

impl<'ctx: 'gd, 'gd, Ctx> Drop for ContextGuard<'ctx, 'gd, Ctx> {
    fn drop(&mut self) {
        (self.guard.1)();
    }
}
