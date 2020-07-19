#![allow(non_upper_case_globals)]

use crate::backends::cdrom::libmirage::*;
use libmirage_sys::*;

pub(crate) fn disc_loaded(backend_params: &BackendParams) -> Result<bool, String> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe { Ok(!DISC.is_null()) }
}

pub(crate) fn disc_mode(backend_params: &BackendParams) -> Result<usize, String> {
    // TODO: mode assumed to be from the first track? Not quite sure...
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if DISC.is_null() {
            return Err("No disc loaded".into());
        }

        let mut _error: *mut GError = std::ptr::null_mut();
        let track = mirage_disc_get_track_by_index(DISC, 0, &mut _error as *mut *mut GError);

        if track.is_null() {
            return Err("No track on disc (null)".into());
        }

        let sector_type = mirage_track_get_sector_type(track);

        match sector_type {
            _MirageSectorType_MIRAGE_SECTOR_MODE2
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM1
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM2
            | _MirageSectorType_MIRAGE_SECTOR_MODE2_MIXED => Ok(2),
            _ => Err(format!("Sector type not implemented: {}", sector_type)),
        }
    }
}

pub(crate) fn read_sector(backend_params: &BackendParams, msf_address_base: (u8, u8, u8), msf_address_offset: usize) -> Result<Vec<u8>, String> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if DISC.is_null() {
            return Err("No disc loaded".into());
        }

        let mut error: *mut GError = std::ptr::null_mut();

        let minute = mirage_helper_bcd2hex(msf_address_base.0 as gint) as guint8;
        let second = mirage_helper_bcd2hex(msf_address_base.1 as gint) as guint8;
        let frame = mirage_helper_bcd2hex(msf_address_base.2 as gint) as guint8;
        let mut lba_address = mirage_helper_msf2lba(minute, second, frame, 1) as usize;
        lba_address += msf_address_offset;

        let mut sector = mirage_disc_get_sector(DISC, lba_address as gint, &mut error as *mut *mut GError);
        if sector.is_null() {
            return Err("Seeking sector from disc failed".into());
        }

        let mut buffer_raw_ptr: *const guint8 = std::ptr::null_mut();
        let mut buffer_raw_size: gint = 0;
        let result = mirage_sector_get_data(sector, &mut buffer_raw_ptr as *mut *const guint8, &mut buffer_raw_size as *mut gint, &mut error as *mut *mut GError);
        if result == 0 {
            return Err("Reading sector from disc failed".into());
        }

        let mut buffer = Vec::with_capacity(buffer_raw_size as usize);
        for offset in 0..(buffer_raw_size as usize) {
            buffer.push(*buffer_raw_ptr.add(offset));
        }

        g_clear_object((&mut sector as *mut *mut MirageSector) as *mut *mut GObject);
        assert!(sector.is_null());

        Ok(buffer)
    }
}
