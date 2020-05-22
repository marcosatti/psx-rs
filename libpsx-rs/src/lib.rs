#![feature(core_intrinsics)]
#![feature(no_more_cas)]
#![feature(thread_spawn_unchecked)]

pub mod backends;
pub(crate) mod debug;
pub(crate) mod executor;
pub(crate) mod system;
pub(crate) mod types;
pub(crate) mod utilities;

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
use executor::Executor;
use std::{
    path::{
        Path,
        PathBuf,
    },
    time::{
        Duration,
        Instant,
    },
};
use system::types::ControllerContext;

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
    pub(crate) state: Box<State>,
    pub(crate) config: Config<'a, 'b>,
    executor: Executor,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: Config<'a, 'b>) -> Core<'a, 'b> {
        log::info!("Initializing libpsx-rs with {} time delta (us) and {} worker threads", config.time_delta.as_micros(), config.worker_threads);

        let mut state = State::new();

        let bios_path = config.workspace_path.join(r"bios/").join(&config.bios_filename);

        State::initialize(&mut state);
        State::load_bios(&mut state, &bios_path);

        let executor = Executor::new(config.worker_threads);

        video::setup(&config.video_backend);
        audio::setup(&config.audio_backend);
        cdrom::setup(&config.cdrom_backend);

        Core {
            state,
            config,
            executor,
        }
    }

    pub fn step(&mut self) {
        let context = ControllerContext {
            state: &self.state,
            video_backend: &self.config.video_backend,
            audio_backend: &self.config.audio_backend,
            cdrom_backend: &self.config.cdrom_backend,
        };

        let time = self.config.time_delta;
        let event = Event::Time(time);

        let timer = Instant::now();
        let benchmark_results = executor::run(&self.executor, &context, event);
        let scope_duration = timer.elapsed();

        debug::benchmark::trace_performance(time, scope_duration, benchmark_results);
    }

    pub fn change_disc(&mut self, path: &Path) {
        backends::cdrom::change_disc(&self.config.cdrom_backend, path);
    }

    pub fn analyze(&mut self) {
        debug::analysis(self);
    }
}

impl<'a: 'b, 'b> Drop for Core<'a, 'b> {
    fn drop(&mut self) {
        video::teardown(&self.config.video_backend);
        audio::teardown(&self.config.audio_backend);
        cdrom::teardown(&self.config.cdrom_backend);
    }
}
