#![feature(core_intrinsics)]

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
    time::Instant,
};
use system::types::ControllerContext;

#[cfg(feature = "serialization")]
const SAVE_STATE_DEFAULT_NAME: &'static str = "save_state_default.bin";

pub struct Config<'a: 'b, 'b> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a, 'b>,
    pub audio_backend: AudioBackend<'a, 'b>,
    pub cdrom_backend: CdromBackend<'a, 'b>,
    pub time_delta: f64,
    pub worker_threads: usize,
}

pub struct Core<'a: 'b, 'b> {
    state: Box<State>,
    config: Config<'a, 'b>,
    executor: Executor,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: Config<'a, 'b>) -> Core<'a, 'b> {
        log::info!("Initializing libpsx-rs with {} time delta (us) and {} worker threads", config.time_delta * 1e6, config.worker_threads);

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

    #[cfg(feature = "serialization")]
    pub fn save_state(&self, name: Option<&str>) -> Result<(), String> {
        log::warn!("GPU framebuffer serialization not implemented, use with caution");
        let encoded: Vec<u8> = bincode::serialize(&self.state).map_err(|e| format!("Error occurred serializing machine state: {}", e).to_owned())?;
        let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
        let mut path = self.config.workspace_path.join(r"saves/");
        std::fs::create_dir_all(&path).unwrap();
        path = path.join(name);
        std::fs::write(&path, encoded).map_err(|e| format!("Unable to write save state file: {}", e).to_owned())
    }

    #[cfg(feature = "serialization")]
    pub fn load_state(&mut self, name: Option<&str>) -> Result<(), String> {
        let name = name.unwrap_or(SAVE_STATE_DEFAULT_NAME);
        let path = self.config.workspace_path.join(r"saves/").join(name);
        let encoded = std::fs::read(&path).map_err(|e| format!("Unable to read save state file: {}", e).to_owned())?;
        let mut state = bincode::deserialize(&encoded).map_err(|e| format!("Error occurred deserializing machine state: {}", e).to_owned())?;
        std::mem::swap(self.state.as_mut(), &mut state);
        Ok(())
    }
}

impl<'a: 'b, 'b> Drop for Core<'a, 'b> {
    fn drop(&mut self) {
        video::teardown(&self.config.video_backend);
        audio::teardown(&self.config.audio_backend);
        cdrom::teardown(&self.config.cdrom_backend);
    }
}
