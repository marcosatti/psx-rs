//! Note: lifetimes are static to avoid propogation to Core... The threadpool is only used with a scoped context.

use crate::{
    debug::benchmark::BenchmarkResults,
    system::{
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
    },
};
use scoped_threadpool::*;
use std::time::Instant;

struct Task {
    controller_name: &'static str,
    controller_fn: fn(&ControllerContext, Event) -> (),
    context: &'static ControllerContext<'static, 'static, 'static>,
    event: Event,
    benchmark_results: &'static BenchmarkResults,
}

impl Task {
    fn new(
        controller_name: &'static str, controller_fn: fn(&ControllerContext, Event) -> (), context: &'static ControllerContext<'_, '_, '_>, event: Event,
        benchmark_results: &'static BenchmarkResults,
    ) -> Task
    {
        Task {
            controller_name,
            controller_fn,
            context,
            event,
            benchmark_results,
        }
    }
}

impl Thunk for Task {
    fn call_once(self) {
        let timer = Instant::now();
        (self.controller_fn)(self.context, self.event);
        let elapsed = timer.elapsed();
        self.benchmark_results.add_result(self.controller_name, elapsed);
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

pub(crate) fn run(executor: &Executor, context: &ControllerContext, event: Event) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();

    let context = unsafe { std::mem::transmute(context) };
    let benchmark_results_ref = unsafe { std::mem::transmute(&benchmark_results) };

    executor.thread_pool.scope::<Task, Task, _>(Some(Task::new("r3000", run_r3000, context, event, benchmark_results_ref)), |s| {
        s.spawn(Task::new("gpu", run_gpu, context, event, benchmark_results_ref));
        s.spawn(Task::new("dmac", run_dmac, context, event, benchmark_results_ref));
        s.spawn(Task::new("spu", run_spu, context, event, benchmark_results_ref));
        s.spawn(Task::new("timers", run_timers, context, event, benchmark_results_ref));
        s.spawn(Task::new("intc", run_intc, context, event, benchmark_results_ref));
        s.spawn(Task::new("padmc", run_padmc, context, event, benchmark_results_ref));
        s.spawn(Task::new("cdrom", run_cdrom, context, event, benchmark_results_ref));
    });

    benchmark_results
}
