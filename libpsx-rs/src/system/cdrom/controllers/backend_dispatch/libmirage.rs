#![allow(non_upper_case_globals)]

use crate::backends::cdrom::libmirage::{
    state::*,
    *,
};
use libmirage_sys::*;

pub(crate) fn disc_loaded(backend_params: &BackendParams) -> bool {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe { !DISC.is_null() }
}

pub fn disc_mode(backend_params: &BackendParams) -> usize {
    // TODO: mode assumed to be from the first track? Not quite sure...
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert!(!DISC.is_null(), "No disc loaded");

        let mut _error: *mut GError = std::ptr::null_mut();
        let track = mirage_disc_get_track_by_index(DISC, 0, &mut _error as *mut *mut GError);
        assert!(!track.is_null());

        let sector_type = mirage_track_get_sector_type(track);

        match sector_type {
            _MirageSectorType_MIRAGE_SECTOR_MODE2
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM1
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM2
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_MIXED => 2,
            _ => unimplemented!("Unknown sector type encountered"),
        }
    }
}

pub fn msf_to_lba_address(backend_params: &BackendParams, minute: u8, second: u8, frame: u8) -> usize {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {  }
}

pub fn read_sector(backend_params: &BackendParams, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> Vec<u8> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert!(!DISC.is_null(), "No disc loaded");

        let mut error: *mut GError = std::ptr::null_mut();

        let mut lba_address = mirage_helper_msf2lba(msf_address_base.0, msf_address_base.1, msf_address_base.2, 1) as usize;
        lba_address += msf_address_offset;

        let mut sector = mirage_disc_get_sector(DISC, lba_address as gint, &mut error as *mut *mut GError);
        assert!(!sector.is_null());
        assert!(error.is_null());

        let mut buffer_raw_ptr: *const guint8 = std::ptr::null_mut();
        let mut buffer_raw_size: gint = 0;
        let result = mirage_sector_get_data(sector, &mut buffer_raw_ptr as *mut *const guint8, &mut buffer_raw_size as *mut gint, &mut error as *mut *mut GError);
        assert!(error.is_null());
        assert!(result != 0);

        let mut buffer = Vec::with_capacity(buffer_raw_size as usize);
        for offset in 0..(buffer_raw_size as usize) {
            buffer.push(*buffer_raw_ptr.add(offset));
        }

        g_clear_object((&mut sector as *mut *mut MirageSector) as *mut *mut GObject);
        assert!(sector.is_null());

        buffer
    }
}
