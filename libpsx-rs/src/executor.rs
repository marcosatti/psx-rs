//! Note: lifetimes are static to avoid propogation to Core... The threadpool is only used with a scoped context.

use crate::system::{
    cdrom::controllers::run as run_cdrom,
    dmac::controllers::run as run_dmac,
    gpu::controllers::run as run_gpu,
    intc::controllers::run as run_intc,
    padmc::controllers::run as run_padmc,
    r3000::controllers::run as run_r3000,
    spu::controllers::run as run_spu,
    timers::controllers::run as run_timers,
    types::{
        ControllerContext,
        Event,
    },
};
use scoped_threadpool::*;

struct Task {
    controller_fn: fn(&ControllerContext, Event) -> (),
    context: &'static ControllerContext<'static, 'static, 'static>,
    event: Event,
}

impl Task {
    fn new(controller_fn: fn(&ControllerContext, Event) -> (), context: &'static ControllerContext<'_, '_, '_>, event: Event) -> Task {
        Task {
            controller_fn,
            context,
            event,
        }
    }
}

impl Thunk for Task {
    fn call_once(self) {
        (self.controller_fn)(self.context, self.event);
    }
}

unsafe impl Send for Task {
}

pub(crate) struct Executor {
    thread_pool: ThreadPool<Task>,
}

impl Executor {
    pub(crate) fn new(pool_size: usize) -> Executor {
        Executor {
            thread_pool: ThreadPool::new(pool_size, 16, "libpsx-rs"),
        }
    }
}

pub(crate) fn run(executor: &Executor, context: &ControllerContext, event: Event) {
    let context = unsafe { std::mem::transmute(context) };

    executor.thread_pool.scope(|s| {
        s.spawn_inplace(Task::new(run_r3000, context, event));
        s.spawn(Task::new(run_gpu, context, event));
        s.spawn(Task::new(run_dmac, context, event));
        s.spawn(Task::new(run_spu, context, event));
        s.spawn(Task::new(run_timers, context, event));
        s.spawn(Task::new(run_intc, context, event));
        s.spawn(Task::new(run_padmc, context, event));
        s.spawn(Task::new(run_cdrom, context, event));
    });
}
