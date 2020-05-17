#![allow(unused_variables)]

#[cfg(libcdio)]
pub mod libcdio;
#[cfg(libmirage)]
pub mod libmirage;

use std::path::Path;

pub enum CdromBackend<'a: 'b, 'b> {
    None,
    #[cfg(libmirage)]
    Libmirage(libmirage::BackendParams<'a, 'b>),
    #[cfg(libcdio)]
    Libcdio(libcdio::BackendParams<'a, 'b>),
    _Phantom(std::marker::PhantomData<(&'a (), &'b ())>),
}

pub(crate) fn setup(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::setup(params),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::setup(params),
        _ => unimplemented!(),
    }
}

pub(crate) fn teardown(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::teardown(params),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::teardown(params),
        _ => unimplemented!(),
    }
}

pub(crate) fn change_disc(cdrom_backend: &CdromBackend, path: &Path) {
    match cdrom_backend {
        CdromBackend::None => panic!(),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::change_disc(params, path),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => libcdio::change_disc(params, path),
        _ => unimplemented!(),
    }
}
