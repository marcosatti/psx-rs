use crate::debug::benchmark::BenchmarkResults;
use crate::system::dmac::controllers::run as run_dmac;
use crate::system::gpu::controllers::run as run_gpu;
use crate::system::intc::controllers::run as run_intc;
use crate::system::padmc::controllers::run as run_padmc;
use crate::system::r3000::controllers::run as run_r3000;
use crate::system::spu::controllers::run as run_spu;
use crate::system::timers::controllers::run as run_timers;
use crate::system::types::{ControllerContext, Event};
use crate::Context;
use rayon::ThreadPool;
use std::sync::atomic::{fence, Ordering};
use std::time::{Duration, Instant};

const USE_MULTITHREADED: bool = true;

pub fn atomic_broadcast(
    executor: &ThreadPool,
    context: &Context,
    event: Event,
) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();

    fence(Ordering::Acquire);

    if USE_MULTITHREADED {
        run_multi_threaded_broadcast(executor, context, event, &benchmark_results);
    } else {
        run_single_threaded_broadcast(context, event, &benchmark_results);
    }

    fence(Ordering::Release);

    benchmark_results
}

fn atomic_run(
    controller_fn: fn(&mut ControllerContext, Event) -> (),
    context: &Context,
    event: Event,
) -> Duration {
    let timer = Instant::now();

    fence(Ordering::Acquire);

    unsafe {
        let mut controller_context = ControllerContext::from_core_context(context);
        controller_fn(&mut controller_context, event);
    }

    fence(Ordering::Release);

    timer.elapsed()
}

fn run_single_threaded_broadcast(
    context: &Context,
    event: Event,
    benchmark_results: &BenchmarkResults,
) {
    let elapsed = atomic_run(run_r3000, context, event);
    benchmark_results.add_result("r3000", elapsed);
    let elapsed = atomic_run(run_dmac, context, event);
    benchmark_results.add_result("dmac", elapsed);
    let elapsed = atomic_run(run_gpu, context, event);
    benchmark_results.add_result("gpu", elapsed);
    let elapsed = atomic_run(run_spu, context, event);
    benchmark_results.add_result("spu", elapsed);
    let elapsed = atomic_run(run_intc, context, event);
    benchmark_results.add_result("intc", elapsed);
    let elapsed = atomic_run(run_padmc, context, event);
    benchmark_results.add_result("padmc", elapsed);
    let elapsed = atomic_run(run_timers, context, event);
    benchmark_results.add_result("timers", elapsed);
}

fn run_multi_threaded_broadcast(
    executor: &ThreadPool,
    context: &Context,
    event: Event,
    benchmark_results: &BenchmarkResults,
) {
    executor.scope(|scope| {
        scope.spawn(|_| {
            let elapsed = atomic_run(run_r3000, context, event);
            benchmark_results.add_result("r3000", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_dmac, context, event);
            benchmark_results.add_result("dmac", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_gpu, context, event);
            benchmark_results.add_result("gpu", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_spu, context, event);
            benchmark_results.add_result("spu", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_intc, context, event);
            benchmark_results.add_result("intc", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_padmc, context, event);
            benchmark_results.add_result("padmc", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_timers, context, event);
            benchmark_results.add_result("timers", elapsed);
        });
    });
}
