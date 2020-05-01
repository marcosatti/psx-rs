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
use futures::future::join_all;
use std::time::Instant;
use tokio::{
    runtime::Runtime,
    spawn,
};

pub fn run_event_broadcast_block(runtime: &mut Runtime, context: &ControllerContext, event: Event) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();

    // Safety: Force the context lifetime to be static (blocking until all tasks have complete).
    unsafe {
        let context: &'static ControllerContext = std::mem::transmute(context);
        let benchmark_results: &'static BenchmarkResults = std::mem::transmute(&benchmark_results);
        runtime.block_on(run_event_broadcast(context, event, &benchmark_results));
    }

    benchmark_results
}

async unsafe fn run_event_broadcast(context: &'static ControllerContext<'_, '_, '_>, event: Event, benchmark_results: &'static BenchmarkResults) {
    let mut tasks = [
        spawn(run_event("r3000", run_r3000, context, event, benchmark_results)),
        spawn(run_event("dmac", run_dmac, context, event, benchmark_results)),
        spawn(run_event("gpu", run_gpu, context, event, benchmark_results)),
        spawn(run_event("spu", run_spu, context, event, benchmark_results)),
        spawn(run_event("intc", run_intc, context, event, benchmark_results)),
        spawn(run_event("padmc", run_padmc, context, event, benchmark_results)),
        spawn(run_event("timers", run_timers, context, event, benchmark_results)),
    ];
    join_all(&mut tasks).await;
}

async unsafe fn run_event(
    controller_name: &'static str, controller_fn: fn(&'static ControllerContext, Event) -> (), context: &'static ControllerContext<'_, '_, '_>, event: Event,
    benchmark_results: &'static BenchmarkResults,
)
{
    let timer = Instant::now();
    controller_fn(context, event);
    let elapsed = timer.elapsed();
    benchmark_results.add_result(controller_name, elapsed);
}
