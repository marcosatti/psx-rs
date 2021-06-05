#![feature(core_intrinsics)]
#![feature(trait_alias)]

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
    system::types::State,
};
use executor::Executor;
pub use executor::ThreadingKind;
use std::{
    io::Result as IoResult,
    path::{
        Path,
        PathBuf,
    },
};
use system::types::ControllerContext;

pub struct Config<'a> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a>,
    pub audio_backend: AudioBackend<'a>,
    pub cdrom_backend: CdromBackend<'a>,
    pub time_delta: f32,
    pub threading: ThreadingKind,
    pub internal_scale_factor: usize,
    pub global_bias: f32,
    pub r3000_bias: f32,
    pub gpu_bias: f32,
    pub gpu_crtc_bias: f32,
    pub dmac_bias: f32,
    pub spu_bias: f32,
    pub timers_bias: f32,
    pub cdrom_bias: f32,
    pub padmc_bias: f32,
    pub intc_bias: f32,
}

pub struct Core<'a: 'b, 'b> {
    pub(crate) state: Box<State>,
    pub(crate) config: &'b Config<'a>,
    executor: Executor,
}

impl<'a: 'b, 'b> Core<'a, 'b> {
    pub fn new(config: &'b Config<'a>) -> IoResult<Core<'a, 'b>> {
        log::info!("Initializing core");

        let state = State::with_bios(&config.workspace_path, &config.bios_filename)?;
        let executor = Executor::new(config.threading);

        video::setup(config);
        audio::setup(config);
        cdrom::setup(config);

        Ok(Core {
            state,
            config,
            executor,
        })
    }

    pub fn reset(&mut self, hard_reset: bool) -> IoResult<()> {
        if hard_reset {
            self.state = State::with_bios(&self.config.workspace_path, &self.config.bios_filename)?;
        } else {
            State::initialize(&mut self.state);
        }

        Ok(())
    }

    pub fn step(&mut self, iterations: usize) -> Result<(), Vec<String>> {
        let context = ControllerContext {
            state: &self.state,
            video_backend: &self.config.video_backend,
            audio_backend: &self.config.audio_backend,
            cdrom_backend: &self.config.cdrom_backend,
        };

        self.executor.run(iterations, &self.config, &context)
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
