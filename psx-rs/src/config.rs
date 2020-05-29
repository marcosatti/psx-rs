use crate::backend::*;
use serde::Deserialize;
use std::{
    fs::File,
    io::prelude::*,
    path::Path,
    time::Duration,
};

#[derive(Deserialize)]
struct TomlConfig {
    audio_backend: String,
    cdrom_backend: String,
    video_backend: String,
    worker_threads: usize,
    time_delta: u64,
    pause_on_exit: bool,
}

pub(crate) struct Config {
    pub(crate) audio_backend_kind: AudioBackendKind,
    pub(crate) cdrom_backend_kind: CdromBackendKind,
    pub(crate) video_backend_kind: VideoBackendKind,
    pub(crate) worker_threads: usize,
    pub(crate) time_delta: Duration,
    pub(crate) pause_on_exit: bool,
}

pub(crate) fn load(workspace_path: &Path) -> Config {
    let config_path = workspace_path.to_owned().join(r"config.toml");
    let mut config_file = File::open(config_path).unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();
    let toml_config: TomlConfig = toml::from_str(&config_str).unwrap();

    Config {
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
        worker_threads: { toml_config.worker_threads },
        time_delta: { Duration::from_micros(toml_config.time_delta as u64) },
        pause_on_exit: toml_config.pause_on_exit,
    }
}
