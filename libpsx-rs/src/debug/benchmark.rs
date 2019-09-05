use std::cell::UnsafeCell;
use std::time::{Duration, Instant};
use log::debug;
use average::{Mean, Estimate};

const ENABLE_BENCHMARK_TRACING: bool = true;

const REPORTING_PERIOD: Duration = Duration::from_secs(3);

pub struct Benchmark {
    pub r3000: UnsafeCell<Duration>,
    pub dmac: UnsafeCell<Duration>,
    pub gpu: UnsafeCell<Duration>,
    pub spu: UnsafeCell<Duration>,
    pub gpu_crtc: UnsafeCell<Duration>,
    pub intc: UnsafeCell<Duration>,
}

impl Benchmark {
    pub const fn empty() -> Benchmark {
        Benchmark {
            r3000: UnsafeCell::new(Duration::from_secs(0)),
            dmac: UnsafeCell::new(Duration::from_secs(0)),
            gpu: UnsafeCell::new(Duration::from_secs(0)),
            spu: UnsafeCell::new(Duration::from_secs(0)),
            gpu_crtc: UnsafeCell::new(Duration::from_secs(0)),
            intc: UnsafeCell::new(Duration::from_secs(0)),
        }
    }
}

unsafe impl Send for Benchmark {}
unsafe impl Sync for Benchmark {}

struct State {
    last_reported: Instant,

    total_host_time_elapsed: Duration,
    average_host_time_elapsed: Mean,

    total_guest_time_elapsed: Duration,
    average_guest_time_elapsed: Mean,

    // Average host time elapsed of individual controllers.
    r3000: Mean,
    dmac: Mean,
    gpu: Mean,
    spu: Mean,
    gpu_crtc: Mean,
    intc: Mean,
}

impl State {
    fn new() -> State {
        State {
            last_reported: Instant::now(),
            total_host_time_elapsed: Duration::from_secs(0),
            average_host_time_elapsed: Mean::new(),
            total_guest_time_elapsed: Duration::from_secs(0),
            average_guest_time_elapsed: Mean::new(),
            r3000: Mean::new(),
            dmac: Mean::new(),
            gpu: Mean::new(),
            spu: Mean::new(),
            gpu_crtc: Mean::new(),
            intc: Mean::new(),
        }
    }
}

static mut BENCHMARK_STATE: Option<State> = None;

pub fn trace_performance(host_time_elapsed: &Duration, guest_time_elapsed: &Duration, benchmark: &Benchmark) {
    if !ENABLE_BENCHMARK_TRACING {
        return;
    }

    unsafe {
        if BENCHMARK_STATE.is_none() {
            BENCHMARK_STATE = Some(State::new());
        }

        let state = BENCHMARK_STATE.as_mut().unwrap();

        state.total_host_time_elapsed += *host_time_elapsed;
        state.average_host_time_elapsed.add(host_time_elapsed.as_secs_f64());
        state.total_guest_time_elapsed += *guest_time_elapsed;
        state.average_guest_time_elapsed.add(guest_time_elapsed.as_secs_f64());
        state.r3000.add((*benchmark.r3000.get()).as_secs_f64());
        state.dmac.add((*benchmark.dmac.get()).as_secs_f64());
        state.gpu.add((*benchmark.gpu.get()).as_secs_f64());
        state.spu.add((*benchmark.spu.get()).as_secs_f64());
        state.gpu_crtc.add((*benchmark.gpu_crtc.get()).as_secs_f64());
        state.intc.add((*benchmark.intc.get()).as_secs_f64());

        if state.last_reported.elapsed() > REPORTING_PERIOD {
            let overall_time_elapsed_percent = state.total_host_time_elapsed.as_secs_f64() / state.total_guest_time_elapsed.as_secs_f64() * 100.0;
            let average_host_time_elapsed = Duration::from_secs_f64(state.average_host_time_elapsed.estimate()).as_micros();
            let average_guest_time_elapsed = Duration::from_secs_f64(state.average_guest_time_elapsed.estimate()).as_micros();
            let average_r3000_time_elapsed = Duration::from_secs_f64(state.r3000.estimate()).as_micros();
            let average_dmac_time_elapsed = Duration::from_secs_f64(state.dmac.estimate()).as_micros();
            let average_gpu_time_elapsed = Duration::from_secs_f64(state.gpu.estimate()).as_micros();
            let average_spu_time_elapsed = Duration::from_secs_f64(state.spu.estimate()).as_micros();
            let average_gpu_crtc_time_elapsed = Duration::from_secs_f64(state.gpu_crtc.estimate()).as_micros();
            let average_intc_time_elapsed = Duration::from_secs_f64(state.intc.estimate()).as_micros();

            debug!(
                "Perf overall {:.2}% (avg {} / {}): r3000 = {}, dmac = {}, gpu = {}, spu = {}, gpu_crtc = {}, intc = {} (units: us)", 
                overall_time_elapsed_percent,
                average_host_time_elapsed,
                average_guest_time_elapsed,
                average_r3000_time_elapsed, average_dmac_time_elapsed, average_gpu_time_elapsed, average_spu_time_elapsed, average_gpu_crtc_time_elapsed, average_intc_time_elapsed
            );

            BENCHMARK_STATE = None;
        }
    }
}
