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
};
use rayon::{
    join,
    ThreadPool,
};
use std::time::Instant;

pub fn run_event_broadcast_block(runtime: &ThreadPool, context: &ControllerContext, event: Event) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();
    let benchmark_results_ref = &benchmark_results;

    runtime.install(|| {
        join(
            move || run_event("r3000", run_r3000, context, event, benchmark_results_ref),
            || {
                join(
                    move || run_event("gpu", run_gpu, context, event, benchmark_results_ref),
                    || {
                        join(
                            move || run_event("dmac", run_dmac, context, event, benchmark_results_ref),
                            || {
                                join(
                                    move || run_event("spu", run_spu, context, event, benchmark_results_ref),
                                    || {
                                        join(
                                            move || run_event("timers", run_timers, context, event, benchmark_results_ref),
                                            || {
                                                join(
                                                    move || run_event("intc", run_intc, context, event, benchmark_results_ref),
                                                    move || run_event("padmc", run_padmc, context, event, benchmark_results_ref),
                                                )
                                            },
                                        )
                                    },
                                )
                            },
                        )
                    },
                )
            },
        );
    });

    benchmark_results
}

fn run_event(controller_name: &'static str, controller_fn: fn(&ControllerContext, Event) -> (), context: &ControllerContext, event: Event, benchmark_results: &BenchmarkResults) {
    let timer = Instant::now();
    controller_fn(context, event);
    let elapsed = timer.elapsed();
    benchmark_results.add_result(controller_name, elapsed);
}
