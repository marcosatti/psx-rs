#![allow(non_upper_case_globals)]

use libmirage_sys::*;
use crate::backends::cdrom::libmirage::*;
use crate::backends::cdrom::libmirage::state::*;

pub fn disc_mode(backend_params: &BackendParams) -> usize {
    // TODO: mode assumed to be from the first track? Not quite sure...
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert!(!DISC.is_null());

        let mut _error: *mut GError = std::ptr::null_mut();
        let track = mirage_disc_get_track_by_index(DISC, 0, &mut _error as *mut *mut GError);
        assert!(!track.is_null());
    
        let sector_type = mirage_track_get_sector_type(track);
    
        match sector_type {
            _MirageSectorType_MIRAGE_SECTOR_MODE2 | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM1 | _MirageSectorType_MIRAGE_SECTOR_MODE2_FORM2 | _MirageSectorType_MIRAGE_SECTOR_MODE2_MIXED => 2,
            _ => unimplemented!("Unknown sector type encountered"),
        }
    }
}
