#![feature(core_intrinsics)]

pub mod backends;
pub(crate) mod debug;
pub(crate) mod executor;
#[cfg(feature = "serialization")]
pub(crate) mod serialization;
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
    io::Result as IoResult,
    path::{
        Path,
        PathBuf,
    },
};
use system::types::ControllerContext;

pub struct Config<'a: 'b, 'b> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a, 'b>,
    pub audio_backend: AudioBackend<'a, 'b>,
    pub cdrom_backend: CdromBackend<'a, 'b>,
    pub time_delta: f64,
    pub worker_threads: usize,
    pub internal_scale_factor: usize,
}

pub struct Core<'a: 'b, 'b> {
    pub(crate) state: Box<State>,
    pub(crate) config: Config<'a, 'b>,
    executor: Executor,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: Config<'a, 'b>) -> IoResult<Core<'a, 'b>> {
        log::info!("Initializing core");

        let state = State::from_bios(&config.workspace_path, &config.bios_filename)?;
        let executor = Executor::new(config.worker_threads);

        video::setup(&config);
        audio::setup(&config);
        cdrom::setup(&config);

        Ok(Core {
            state,
            config,
            executor,
        })
    }

    pub fn reset(&mut self, hard_reset: bool) -> IoResult<()> {
        if hard_reset {
            self.state = State::from_bios(&self.config.workspace_path, &self.config.bios_filename)?;
        } else {
            State::initialize(&mut self.state);
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Vec<String>> {
        let context = ControllerContext {
            state: &self.state,
            video_backend: &self.config.video_backend,
            audio_backend: &self.config.audio_backend,
            cdrom_backend: &self.config.cdrom_backend,
        };

        let event = Event::Time(self.config.time_delta);

        self.executor.run(&context, event)
    }

    pub fn change_disc(&mut self, path: &Path) -> Result<(), String> {
        backends::cdrom::change_disc(&self.config, path)
    }

    pub fn analyze(&mut self) -> IoResult<()> {
        debug::analysis(self)
    }

    #[cfg(feature = "serialization")]
    pub fn save_state(&self, name: Option<&str>) -> Result<(), String> {
        serialization::save_state(self, name)
    }

    #[cfg(feature = "serialization")]
    pub fn load_state(&mut self, name: Option<&str>) -> Result<(), String> {
        serialization::load_state(self, name)
    }
}

impl<'a: 'b, 'b> Drop for Core<'a, 'b> {
    fn drop(&mut self) {
        video::teardown(&self.config);
        audio::teardown(&self.config);
        cdrom::teardown(&self.config);
    }
}
