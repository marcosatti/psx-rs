pub mod libmirage;

use crate::backends::cdrom::libmirage::*;

pub enum CdromBackend<'a> {
    None,
    Libmirage(BackendParams<'a>),
}

pub fn setup(cdrom_backend: &CdromBackend) {
    match cdrom_backend {
        CdromBackend::None => {},
        CdromBackend::Libmirage(ref params) => libmirage::setup(params),
    }
}
