#![feature(core_intrinsics)]

pub mod constants;
pub mod types;
pub mod utilities;
pub mod resources;
pub mod controllers;
pub mod debug;
pub mod backends;
pub mod executor;

use std::pin::Pin;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use rayon::{ThreadPool, ThreadPoolBuilder};
use log::info;
use crate::backends::video::{self, VideoBackend};
use crate::backends::audio::{self, AudioBackend};
use crate::resources::Resources;
use crate::controllers::Event;

pub struct State<'b, 'a: 'b> {
    pub resources: *mut Resources,
    pub video_backend: &'b VideoBackend<'a>,
    pub audio_backend: &'b AudioBackend<'a>,
}

unsafe impl<'b, 'a> Sync for State<'b, 'a> {}

pub struct Config<'a> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a>,
    pub audio_backend: AudioBackend<'a>,
    pub time_delta: Duration,
    pub worker_threads: usize,
}

pub struct Core<'a> {
    pub resources: Pin<Box<Resources>>,
    task_executor: ThreadPool,
    config: Config<'a>,
}

impl<'a> Core<'a> {
    pub fn new(config: Config) -> Core {
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
        };

        let time = self.config.time_delta;
        let event = Event::Time(time);
        
        let timer = Instant::now();
        let benchmark_results = executor::atomic_broadcast(&self.task_executor, &state, event);
        let scope_duration = timer.elapsed();

        debug::benchmark::trace_performance(time, scope_duration, benchmark_results);
    }
}
