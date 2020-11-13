#![allow(unused_variables)]

#[cfg(libcdio)]
pub mod libcdio;
#[cfg(libmirage)]
pub mod libmirage;

use crate::Config;
use std::path::Path;

pub enum CdromBackend<'a> {
    None,
    #[cfg(libmirage)]
    Libmirage(libmirage::BackendParams<'a>),
    #[cfg(libcdio)]
    Libcdio(libcdio::BackendParams<'a>),
    _Phantom(std::marker::PhantomData<&'a ()>),
}

pub(crate) fn setup(config: &Config) {
    match config.cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::setup(config, params),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::setup(config, params),
        _ => unimplemented!(),
    }
}

pub(crate) fn teardown(config: &Config) {
    match config.cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::teardown(config, params),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::teardown(config, params),
        _ => unimplemented!(),
    }
}

pub(crate) fn change_disc(config: &Config, path: &Path) -> Result<(), String> {
    match config.cdrom_backend {
        CdromBackend::None => Err("No available backend".into()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::change_disc(config, params, path),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::change_disc(config, params, path),
        _ => unimplemented!(),
    }
}
