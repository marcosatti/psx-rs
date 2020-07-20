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
        ControllerHandler,
        ControllerResult,
        Event,
    },
};
use crossbeam::channel::bounded;
use rayon::{
    ThreadPool,
    ThreadPoolBuilder,
};
use crate::Config;

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

    pub(crate) fn run(&self, config: &Config, context: &ControllerContext) -> Result<(), Vec<String>> {
        const CONTROLLERS_COUNT: usize = 8;

        let time_delta = config.time_delta * config.global_bias;
        let r3000_event = Event::Time(time_delta * config.r3000_bias);
        let gpu_event = Event::Time(time_delta * config.gpu_bias);
        let dmac_event = Event::Time(time_delta * config.dmac_bias);
        let spu_event = Event::Time(time_delta * config.spu_bias);
        let timers_event = Event::Time(time_delta * config.timers_bias);
        let cdrom_event = Event::Time(time_delta * config.cdrom_bias);
        let padmc_event = Event::Time(time_delta * config.padmc_bias);
        let intc_event = Event::Time(time_delta * config.intc_bias);

        let (sender, collector) = bounded(CONTROLLERS_COUNT);

        self.thread_pool.scope(|s| {
            s.spawn(|_| sender.send(run_controller_proxy("intc", run_intc, context, intc_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("padmc", run_padmc, context, padmc_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("cdrom", run_cdrom, context, cdrom_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("timers", run_timers, context, timers_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("spu", run_spu, context, spu_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("dmac", run_dmac, context, dmac_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("gpu", run_gpu, context, gpu_event)).unwrap());
            s.spawn(|_| sender.send(run_controller_proxy("r3000", run_r3000, context, r3000_event)).unwrap());
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

fn run_controller_proxy(name: &str, handler_fn: ControllerHandler, context: &ControllerContext, event: Event) -> ControllerResult<()> {
    handler_fn(context, event).map_err(|what| format!("{}: {}", name, &what))
}
