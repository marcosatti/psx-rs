use crate::{
    debug::benchmark::BenchmarkResults,
    system::{
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
    Context,
};
use futures_util::try_join;
use std::{
    sync::atomic::{
        fence,
        Ordering,
    },
    time::{
        Duration,
        Instant,
    },
};
use tokio::{
    runtime::Runtime,
    spawn,
};

pub fn atomic_broadcast(runtime: &mut Runtime, context: &Context, event: Event) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();

    fence(Ordering::Acquire);

    run_broadcast(runtime, context, event, &benchmark_results);

    fence(Ordering::Release);

    benchmark_results
}

fn run_broadcast(runtime: &mut Runtime, context: &Context, event: Event, benchmark_results: &BenchmarkResults) {
    // Use of unsafe to force the context lifetime to be static (blocking until all tasks have complete).

    let context: &'static Context = unsafe { std::mem::transmute(context) };

    let benchmark_results: &'static BenchmarkResults = unsafe { std::mem::transmute(benchmark_results) };

    runtime.block_on(async move {
        let result = try_join!(
            spawn(async move {
                let elapsed = atomic_run(run_r3000, context, event);
                benchmark_results.add_result("r3000", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_dmac, context, event);
                benchmark_results.add_result("dmac", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_gpu, context, event);
                benchmark_results.add_result("gpu", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_spu, context, event);
                benchmark_results.add_result("spu", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_intc, context, event);
                benchmark_results.add_result("intc", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_padmc, context, event);
                benchmark_results.add_result("padmc", elapsed);
            }),
            spawn(async move {
                let elapsed = atomic_run(run_timers, context, event);
                benchmark_results.add_result("timers", elapsed);
            }),
        );

        result.unwrap();
    });
}

fn atomic_run(controller_fn: fn(&mut ControllerContext, Event) -> (), context: &Context, event: Event) -> Duration {
    let timer = Instant::now();

    fence(Ordering::Acquire);

    unsafe {
        let mut controller_context = ControllerContext::from_core_context(context);
        controller_fn(&mut controller_context, event);
    }

    fence(Ordering::Release);

    timer.elapsed()
}
