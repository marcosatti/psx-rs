#![allow(unused_variables)]

#[cfg(libmirage)]
mod libmirage;

use crate::backends::cdrom::CdromBackend;

pub(crate) fn msf_to_lba(cdrom_backend: &CdromBackend, minute: u8, second: u8, frame: u8) -> usize {
    match cdrom_backend {
        CdromBackend::None => panic!(),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::msf_to_lba_address(params, minute, second, frame),
        _ => unimplemented!(),
    }
}

pub(crate) fn disc_mode(cdrom_backend: &CdromBackend) -> usize {
    match cdrom_backend {
        CdromBackend::None => panic!(),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::disc_mode(params),
        _ => unimplemented!(),
    }
}

pub(crate) fn read_sector(cdrom_backend: &CdromBackend, lba_address: usize) -> Vec<u8> {
    match cdrom_backend {
        CdromBackend::None => panic!(),
        #[cfg(libmirage)]
        CdromBackend::Libmirage(ref params) => libmirage::read_sector(params, resources.cdrom.lba_address),
        _ => unimplemented!(),
    }
}
