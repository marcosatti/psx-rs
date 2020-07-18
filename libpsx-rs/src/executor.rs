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
        Event, ControllerHandler, ControllerResult,
    },
};
use crossbeam::channel::bounded;
use rayon::{
    ThreadPool,
    ThreadPoolBuilder,
};

pub(crate) struct Executor {
    thread_pool: ThreadPool,
}

impl Executor {
    pub(crate) fn new(pool_size: usize) -> Executor {
        let thread_pool = ThreadPoolBuilder::new().num_threads(pool_size).thread_name(|i| format!("libpsx-rs-worker-{}", i)).build().unwrap();

        Executor {
            thread_pool,
        }
    }

    pub(crate) fn run(&self, context: &ControllerContext, event: Event) -> Result<(), Vec<String>> {
        const CONTROLLERS_COUNT: usize = 8;

        let (sender, collector) = bounded(CONTROLLERS_COUNT);

        self.thread_pool.scope(|s| {
            s.spawn(|_| sender.send(run_controller_proxy("intc", run_intc, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("padmc", run_padmc, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("cdrom", run_cdrom, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("timers", run_timers, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("spu", run_spu, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("dmac", run_dmac, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("gpu", run_gpu, context, event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("r3000", run_r3000, context, event)).unwrap());
        });

        let mut results: Vec<String> = Vec::new();

        for _ in 0..CONTROLLERS_COUNT {
            let result = collector.try_recv().unwrap();
            if result.is_err() {
                results.push(result.unwrap_err());
            }
        }

        if !results.is_empty() {
            Err(results)
        } else {
            Ok(())
        }
    }
}

fn run_controller_proxy(name: &str, handler_fn: ControllerHandler, context: &ControllerContext, event: Event) -> ControllerResult {
    handler_fn(context, event).map_err(|what| {
        format!("{}: {}", name, &what)
    })
}
