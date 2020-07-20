#![allow(unused_variables)]

#[cfg(libcdio)]
mod libcdio;
#[cfg(libmirage)]
mod libmirage;

use crate::backends::cdrom::CdromBackend;
use crate::system::types::ControllerResult;

pub(crate) fn disc_loaded(cdrom_backend: &CdromBackend) -> ControllerResult<Result<bool, ()>> {
    match cdrom_backend {
        CdromBackend::None => Ok(Err(())),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(Ok(libmirage::disc_loaded(params)?)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(Ok(libcdio::disc_loaded(params)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn disc_mode(cdrom_backend: &CdromBackend) -> ControllerResult<Result<usize, ()>> {
    match cdrom_backend {
        CdromBackend::None => Ok(Err(())),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(Ok(libmirage::disc_mode(params)?)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(Ok(libcdio::disc_mode(params)?)),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_sector(cdrom_backend: &CdromBackend, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> ControllerResult<Result<Vec<u8>, ()>> {
    match cdrom_backend {
        CdromBackend::None => Ok(Err(())),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => Ok(Ok(libmirage::read_sector(params, msf_address_base, msf_address_offset)?)),
        #[cfg(libcdio)]
        CdromBackend::Libcdio(ref params) => Ok(Ok(libcdio::read_sector(params, msf_address_base, msf_address_offset)?)),
        _ => unimplemented!(),
    }
}
