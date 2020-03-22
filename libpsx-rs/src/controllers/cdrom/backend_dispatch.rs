#![allow(unused_variables)]

#[cfg(libmirage)]
mod libmirage;

use crate::backends::cdrom::CdromBackend;

pub(crate) fn disc_loaded(cdrom_backend: &CdromBackend) -> Result<bool, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::disc_loaded(params)),
        _ => unimplemented!(),
    }
}

pub(crate) fn msf_to_lba(cdrom_backend: &CdromBackend, minute: u8, second: u8, frame: u8) -> Result<usize, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::msf_to_lba_address(params, minute, second, frame)),
        _ => unimplemented!(),
    }
}

pub(crate) fn disc_mode(cdrom_backend: &CdromBackend) -> Result<usize, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::disc_mode(params)),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_sector(cdrom_backend: &CdromBackend, lba_address: usize) -> Result<Vec<u8>, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::read_sector(params, lba_address)),
        _ => unimplemented!(),
    }
}
