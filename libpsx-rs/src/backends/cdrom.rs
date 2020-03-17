#[cfg(libmirage)]
pub mod libmirage;

use std::path::Path;

#[cfg(any(libmirage))]
pub enum CdromBackend<'a> {
    None,
    #[cfg(libmirage)]
    Libmirage(libmirage::BackendParams<'a>),
}

#[cfg(not(any(libmirage)))]
pub enum CdromBackend {
    None,
}

pub fn setup(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::setup(params),
    }
}

pub fn teardown(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::teardown(params),
    }
}

pub fn change_disc(cdrom_backend: &CdromBackend, path: &Path) {
    match cdrom_backend {
        CdromBackend::None => panic!("No CDROM handler loaded"),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::change_disc(params, path),
    }
}
