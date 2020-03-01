pub mod libmirage;

use std::path::Path;

pub enum CdromBackend<'a> {
    None,
    Libmirage(libmirage::BackendParams<'a>),
}

pub fn setup(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        CdromBackend::Libmirage(ref params) => libmirage::setup(params),
    }
}

pub fn teardown(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        CdromBackend::Libmirage(ref params) => libmirage::teardown(params),
    }
}

pub fn change_disc(cdrom_backend: &CdromBackend, path: &Path) {
    match cdrom_backend {
        CdromBackend::None => panic!("No CDROM handler loaded"),
        CdromBackend::Libmirage(ref params) => libmirage::change_disc(params, path),
    }
}
