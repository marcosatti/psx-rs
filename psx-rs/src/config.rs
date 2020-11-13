use crate::backend::*;
use serde::Deserialize;
use std::{
    fs::File,
    io::prelude::*,
    path::Path,
};

#[derive(Deserialize)]
struct TomlConfig {
    sdl2_force_wayland_video_driver: bool,
    audio_backend: String,
    cdrom_backend: String,
    video_backend: String,
    worker_threads: usize,
    time_delta: u64,
    pause_on_start: bool,
    quit_on_exception: bool,
    internal_scale_factor: usize,
    global_bias: f64,
    r3000_bias: f64,
    gpu_bias: f64,
    gpu_crtc_bias: f64,
    dmac_bias: f64,
    spu_bias: f64,
    timers_bias: f64,
    cdrom_bias: f64,
    padmc_bias: f64,
    intc_bias: f64,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Config {
    pub(crate) sdl2_force_wayland_video_driver: bool,
    pub(crate) audio_backend_kind: AudioBackendKind,
    pub(crate) cdrom_backend_kind: CdromBackendKind,
    pub(crate) video_backend_kind: VideoBackendKind,
    pub(crate) worker_threads: usize,
    pub(crate) time_delta_secs: f64,
    pub(crate) pause_on_start: bool,
    pub(crate) quit_on_exception: bool,
    pub(crate) internal_scale_factor: usize,
    pub(crate) global_bias: f64,
    pub(crate) r3000_bias: f64,
    pub(crate) gpu_bias: f64,
    pub(crate) gpu_crtc_bias: f64,
    pub(crate) dmac_bias: f64,
    pub(crate) spu_bias: f64,
    pub(crate) timers_bias: f64,
    pub(crate) cdrom_bias: f64,
    pub(crate) padmc_bias: f64,
    pub(crate) intc_bias: f64,
}

pub(crate) fn load(workspace_path: &Path) -> Config {
    let config_path = workspace_path.to_owned().join(r"config.toml");
    let mut config_file = File::open(config_path).unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();
    let toml_config: TomlConfig = toml::from_str(&config_str).unwrap();

    Config {
        sdl2_force_wayland_video_driver: toml_config.sdl2_force_wayland_video_driver,
        audio_backend_kind: {
            match toml_config.audio_backend.as_ref() {
                "none" => AudioBackendKind::None,
                "openal" => AudioBackendKind::Openal,
                _ => panic!("Unrecongnised config option for the audio backend"),
            }
        },
        cdrom_backend_kind: {
            match toml_config.cdrom_backend.as_ref() {
                "none" => CdromBackendKind::None,
                "libmirage" => CdromBackendKind::Libmirage,
                "libcdio" => CdromBackendKind::Libcdio,
                _ => panic!("Unrecongnised config option for the cdrom backend"),
            }
        },
        video_backend_kind: {
            match toml_config.video_backend.as_ref() {
                "none" => VideoBackendKind::None,
                "opengl" => VideoBackendKind::Opengl,
                _ => panic!("Unrecongnised config option for the video backend"),
            }
        },
        worker_threads: toml_config.worker_threads,
        time_delta_secs: { toml_config.time_delta as f64 / 1e6 },
        pause_on_start: toml_config.pause_on_start,
        quit_on_exception: toml_config.quit_on_exception,
        internal_scale_factor: { toml_config.internal_scale_factor.max(1) },
        global_bias: toml_config.global_bias,
        r3000_bias: toml_config.r3000_bias,
        gpu_bias: toml_config.gpu_bias,
        gpu_crtc_bias: toml_config.gpu_crtc_bias,
        dmac_bias: toml_config.dmac_bias,
        spu_bias: toml_config.spu_bias,
        timers_bias: toml_config.timers_bias,
        cdrom_bias: toml_config.cdrom_bias,
        padmc_bias: toml_config.padmc_bias,
        intc_bias: toml_config.intc_bias,
    }
}
