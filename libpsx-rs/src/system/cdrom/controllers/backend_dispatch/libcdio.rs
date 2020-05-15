#![allow(non_upper_case_globals)]

use crate::backends::cdrom::libcdio::*;
use libcdio_sys::*;

pub(crate) fn disc_loaded(backend_params: &BackendParams) -> bool {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe { 
        !DISC.is_null() 
    }
}

pub fn disc_mode(backend_params: &BackendParams) -> usize {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert!(!DISC.is_null(), "No disc loaded");

        let disc_mode = cdio_get_discmode(DISC);

        match disc_mode {
            discmode_t_CDIO_DISC_MODE_CD_XA => 2,
            _ => unimplemented!(),
        }
    }
}

pub fn read_sector(backend_params: &BackendParams, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> Vec<u8> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert!(!DISC.is_null(), "No disc loaded");

        let msf = msf_s {
            m: msf_address_base.0,
            s: msf_address_base.1,
            f: msf_address_base.2,
        };
        let mut lsn = cdio_msf_to_lsn(&msf);
        lsn += msf_address_offset as i32;

        let read_mode = cdio_read_mode_t_CDIO_READ_MODE_M2F1;
        let length = 2048;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        let result = cdio_read_sector(DISC, buffer.as_mut_ptr() as *mut std::ffi::c_void, lsn, read_mode);
        assert_eq!(result, 0);

        buffer.set_len(length);

        buffer
    }
}
