#![allow(non_upper_case_globals)]

use crate::backends::cdrom::libcdio::*;
use libcdio_sys::*;
use crate::system::types::ControllerResult;

pub(crate) fn disc_loaded(backend_params: &BackendParams) -> ControllerResult<bool> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe { Ok(!DISC.is_null()) }
}

pub(crate) fn disc_mode(backend_params: &BackendParams) -> ControllerResult<usize> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if DISC.is_null() {
            return Err("No disc loaded".into());
        }

        let disc_mode = cdio_get_discmode(DISC);

        match disc_mode {
            discmode_t_CDIO_DISC_MODE_CD_XA => Ok(2),
            _ => Err(format!("Disc mode not implemented: {}", disc_mode)),
        }
    }
}

pub(crate) fn read_sector(backend_params: &BackendParams, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> ControllerResult<Vec<u8>> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if DISC.is_null() {
            return Err("No disc loaded".into());
        }

        let msf = msf_s {
            m: msf_address_base.0,
            s: msf_address_base.1,
            f: msf_address_base.2,
        };
        let mut lsn = cdio_msf_to_lsn(&msf);
        lsn += msf_address_offset as i32;

        let read_mode = cdio_read_mode_t_CDIO_READ_MODE_M2F1;
        let mut buffer = vec![0; 2048];

        let result = cdio_read_sector(DISC, buffer.as_mut_ptr() as *mut std::ffi::c_void, lsn, read_mode);
        if result > 0 {
            return Err(format!("Error reading disc sector; return code: {}", result));
        }

        Ok(buffer)
    }
}
