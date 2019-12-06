use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use log::debug;
use average::{Mean, Estimate};

const ENABLE_BENCHMARK_TRACING: bool = false;

const REPORTING_PERIOD: Duration = Duration::from_secs(3);

pub struct BenchmarkResults {
    results: Mutex<HashMap<&'static str, Duration>>,
}

impl BenchmarkResults {
    pub fn new() -> BenchmarkResults {
        BenchmarkResults {
            results: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_result(&self, controller: &'static str, duration: Duration) {
        self.results.lock().unwrap().insert(controller, duration);
    }

    fn consume(self) -> HashMap<&'static str, Duration> {
        self.results.into_inner().unwrap()
    }
}

struct State {
    last_reported: Instant,

    total_host_time_elapsed: Duration,
    average_host_time_elapsed: Mean,

    total_guest_time_elapsed: Duration,
    average_guest_time_elapsed: Mean,

    average_host_time_elapsed_controllers: HashMap<&'static str, Mean>,
}

impl State {
    fn new() -> State {
        State {
            last_reported: Instant::now(),
            total_host_time_elapsed: Duration::from_secs(0),
            average_host_time_elapsed: Mean::new(),
            total_guest_time_elapsed: Duration::from_secs(0),
            average_guest_time_elapsed: Mean::new(),
            average_host_time_elapsed_controllers: HashMap::new(),
        }
    }
}

static mut BENCHMARK_STATE: Option<State> = None;

pub fn trace_performance(host_time_elapsed: Duration, guest_time_elapsed: Duration, benchmark: BenchmarkResults) {
    if !ENABLE_BENCHMARK_TRACING {
        return;
    }

    unsafe {
        if BENCHMARK_STATE.is_none() {
            BENCHMARK_STATE = Some(State::new());
        }

        let state = BENCHMARK_STATE.as_mut().unwrap();

        state.total_host_time_elapsed += host_time_elapsed;
        state.average_host_time_elapsed.add(host_time_elapsed.as_secs_f64());
        state.total_guest_time_elapsed += guest_time_elapsed;
        state.average_guest_time_elapsed.add(guest_time_elapsed.as_secs_f64());
        
        for controller_result in benchmark.consume().iter() {
            state.average_host_time_elapsed_controllers
                .entry(controller_result.0)
                .or_insert_with(|| Mean::new())
                .add(controller_result.1.as_secs_f64());
        }

        if state.last_reported.elapsed() > REPORTING_PERIOD {
            let overall_time_elapsed_percent = state.total_host_time_elapsed.as_secs_f64() / state.total_guest_time_elapsed.as_secs_f64() * 100.0;
            let average_host_time_elapsed = Duration::from_secs_f64(state.average_host_time_elapsed.estimate()).as_micros();
            let average_guest_time_elapsed = Duration::from_secs_f64(state.average_guest_time_elapsed.estimate()).as_micros();
            
            let mut controller_results_str = Vec::with_capacity(state.average_host_time_elapsed_controllers.len());
            for controller_result in state.average_host_time_elapsed_controllers.iter() {
                controller_results_str.push(format!("{} = {}", controller_result.0, Duration::from_secs_f64(controller_result.1.estimate()).as_micros()))
            }

            controller_results_str.sort_unstable();

            debug!(
                "Perf overall {:.2}% (avg {} / {}): {} (units: us)", 
                overall_time_elapsed_percent,
                average_host_time_elapsed,
                average_guest_time_elapsed,
                controller_results_str.join(", ")
            );

            BENCHMARK_STATE = None;
        }
    }
}
