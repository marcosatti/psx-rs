#![allow(unused_variables)]

#[cfg(libmirage)]
mod libmirage;
#[cfg(libcdio)]
mod libcdio;

use crate::backends::cdrom::CdromBackend;

pub(crate) fn disc_loaded(cdrom_backend: &CdromBackend) -> Result<bool, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::disc_loaded(params)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(libcdio::disc_loaded(params)),
        _ => unimplemented!(),
    }
}

pub(crate) fn disc_mode(cdrom_backend: &CdromBackend) -> Result<usize, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::disc_mode(params)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(libcdio::disc_mode(params)),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_sector(cdrom_backend: &CdromBackend, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> Result<Vec<u8>, ()> {
    match cdrom_backend {
        CdromBackend::None => Err(()),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(libmirage::read_sector(params, msf_address_base, msf_address_offset)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(libcdio::read_sector(params, msf_address_base, msf_address_offset)),
        _ => unimplemented!(),
    }
}
