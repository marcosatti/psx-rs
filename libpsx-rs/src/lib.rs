#![feature(core_intrinsics)]
#![feature(no_more_cas)]

pub mod system;
pub mod types;
pub mod utilities;
pub mod debug;
pub mod backends;
pub mod executor;

use std::pin::Pin;
use std::path::{PathBuf, Path};
use std::time::{Duration, Instant};
use rayon::{ThreadPool, ThreadPoolBuilder};
use log::info;
use crate::backends::video::{self, VideoBackend};
use crate::backends::audio::{self, AudioBackend};
use crate::backends::cdrom::{self, CdromBackend};
use crate::resources::Resources;
use crate::controllers::Event;

pub struct State<'a: 'b, 'b: 'c, 'c> {
    pub resources: *mut Resources,
    pub video_backend: &'c VideoBackend<'a, 'b>,
    pub audio_backend: &'c AudioBackend<'a, 'b>,
    pub cdrom_backend: &'c CdromBackend<'a, 'b>,
}

unsafe impl<'a: 'b, 'b: 'c, 'c> Sync for State<'a, 'b, 'c> {}

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
    pub resources: Pin<Box<Resources>>,
    task_executor: ThreadPool,
    config: Config<'a, 'b>,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: Config<'a, 'b>) -> Core<'a, 'b> {
        info!("Initializing libpsx-rs with {} time delta (us) and {} worker threads", config.time_delta.as_micros(), config.worker_threads);
        info!("Main thread ID: {}", thread_id::get());

        let mut resources = Resources::new();

        let bios_path = config.workspace_path.join(r"bios/").join(&config.bios_filename);

        unsafe {
            let resources_mut = resources.as_mut().get_unchecked_mut();
            Resources::initialize(resources_mut);
            Resources::load_bios(resources_mut, &bios_path);
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
            resources: resources,
            task_executor: task_executor,
            config: config,
        }
    }

    pub fn step(&mut self) {
        let resources_mut = unsafe { 
            self.resources.as_mut().get_unchecked_mut()  
        };
        
        let state = State {
            resources: resources_mut as *mut Resources,
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
