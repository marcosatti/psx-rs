#![feature(core_intrinsics)]
#![feature(no_more_cas)]

pub mod backends;
pub mod debug;
pub mod executor;
pub mod system;
pub mod types;
pub mod utilities;

use crate::{
    backends::{
        audio::{
            self,
            AudioBackend,
        },
        cdrom::{
            self,
            CdromBackend,
        },
        video::{
            self,
            VideoBackend,
        },
    },
    system::types::{
        Event,
        State,
    },
};
use log::info;
use rayon::{
    ThreadPool,
    ThreadPoolBuilder,
};
use std::{
    path::{
        Path,
        PathBuf,
    },
    pin::Pin,
    time::{
        Duration,
        Instant,
    },
};

pub struct Context<'a: 'b, 'b: 'c, 'c> {
    pub state: *mut State,
    pub video_backend: &'c VideoBackend<'a, 'b>,
    pub audio_backend: &'c AudioBackend<'a, 'b>,
    pub cdrom_backend: &'c CdromBackend<'a, 'b>,
}

unsafe impl<'a: 'b, 'b: 'c, 'c> Sync for Context<'a, 'b, 'c> {
}

pub struct Config<'a: 'b, 'b> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a, 'b>,
    pub audio_backend: AudioBackend<'a, 'b>,
    pub cdrom_backend: CdromBackend<'a, 'b>,
    pub time_delta: Duration,
    pub worker_threads: usize,
}

pub struct Core<'a: 'b, 'b> {
    pub state: Pin<Box<State>>,
    task_executor: ThreadPool,
    config: Config<'a, 'b>,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: Config<'a, 'b>) -> Core<'a, 'b> {
        info!(
            "Initializing libpsx-rs with {} time delta (us) and {} worker threads",
            config.time_delta.as_micros(),
            config.worker_threads
        );
        info!("Main thread ID: {}", thread_id::get());

        let mut state = State::new();

        let bios_path = config.workspace_path.join(r"bios/").join(&config.bios_filename);

        unsafe {
            let state_mut = state.as_mut().get_unchecked_mut();
            State::initialize(state_mut);
            State::load_bios(state_mut, &bios_path);
        }

        let task_executor = ThreadPoolBuilder::new()
            .num_threads(config.worker_threads)
            .thread_name(|id| format!("libpsx-rs:{}:{}", thread_id::get(), id))
            .start_handler(|_| {
                info!("Worker thread ID: {:?}", thread_id::get());
            })
            .build()
            .unwrap();

        video::setup(&config.video_backend);
        audio::setup(&config.audio_backend);
        cdrom::setup(&config.cdrom_backend);

        Core {
            state,
            task_executor,
            config,
        }
    }

    pub fn step(&mut self) {
        let state_mut = unsafe { self.state.as_mut().get_unchecked_mut() };

        let state = Context {
            state: state_mut as *mut State,
            video_backend: &self.config.video_backend,
            audio_backend: &self.config.audio_backend,
            cdrom_backend: &self.config.cdrom_backend,
        };

        let time = self.config.time_delta;
        let event = Event::Time(time);

        let timer = Instant::now();
        let benchmark_results = executor::atomic_broadcast(&self.task_executor, &state, event);
        let scope_duration = timer.elapsed();

        debug::benchmark::trace_performance(time, scope_duration, benchmark_results);
    }

    pub fn change_disc(&mut self, path: &Path) {
        backends::cdrom::change_disc(&self.config.cdrom_backend, path);
    }
}

impl<'a: 'b, 'b> Drop for Core<'a, 'b> {
    fn drop(&mut self) {
        video::teardown(&self.config.video_backend);
        audio::teardown(&self.config.audio_backend);
        cdrom::teardown(&self.config.cdrom_backend);
    }
}
