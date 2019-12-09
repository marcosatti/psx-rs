use std::sync::atomic::{fence, Ordering};
use std::time::{Instant, Duration};
use rayon::ThreadPool;
use crate::State;
use crate::controllers::ControllerState;
use crate::controllers::Event;
use crate::debug::benchmark::BenchmarkResults;
use crate::controllers::r3000::run as run_r3000;
use crate::controllers::gpu::crtc::run as run_gpu_crtc;
use crate::controllers::intc::run as run_intc;
use crate::controllers::gpu::run as run_gpu;
use crate::controllers::dmac::run as run_dmac;
use crate::controllers::spu::run as run_spu;
use crate::controllers::padmc::run as run_padmc;
use crate::controllers::timers::run as run_timers;

pub fn atomic_broadcast(executor: &ThreadPool, state: &State, event: Event) -> BenchmarkResults {
    let benchmark_results = BenchmarkResults::new();

    fence(Ordering::Acquire);
    
    executor.scope(|scope| {
        scope.spawn(|_| {
            let elapsed = atomic_run(run_r3000, &state, event);
            benchmark_results.add_result("r3000", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_dmac, &state, event);
            benchmark_results.add_result("dmac", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_gpu, &state, event);
            benchmark_results.add_result("gpu", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_spu, &state, event);
            benchmark_results.add_result("spu", elapsed);
        });
        scope.spawn(|_| {
            let elapsed = atomic_run(run_gpu_crtc, &state, event);
            benchmark_results.add_result("gpu_crtc", elapsed);
        });
        scope.spawn(|_| { 
            let elapsed = atomic_run(run_intc, &state, event);
            benchmark_results.add_result("intc", elapsed);
        });
        scope.spawn(|_| { 
            let elapsed = atomic_run(run_padmc, &state, event);
            benchmark_results.add_result("padmc", elapsed);
        });
        scope.spawn(|_| { 
            let elapsed = atomic_run(run_timers, &state, event);
            benchmark_results.add_result("timers", elapsed);
        });
    });

    fence(Ordering::Release);

    benchmark_results
}

pub fn atomic_run(controller_fn: fn(&mut ControllerState, Event) -> (), state: &State, event: Event) -> Duration {
    let timer = Instant::now();
    
    fence(Ordering::Acquire);

    unsafe {
        let mut controller_state = ControllerState::from_core_state(state);
        controller_fn(&mut controller_state, event);
    }

    fence(Ordering::Release);

    timer.elapsed()
}
