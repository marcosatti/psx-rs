use crate::{
    backends::context::*,
    Config,
};
use libcdio_sys::*;
use log::info;
use std::{
    ffi::CString,
    path::Path,
};

static mut INITIALIZED: bool = false;

pub(crate) static mut DISC: *mut CdIo_t = std::ptr::null_mut();

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, ()>,
}

pub(crate) fn setup(_config: &Config, backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        DISC = std::ptr::null_mut();

        INITIALIZED = true;
    }
}

pub(crate) fn teardown(_config: &Config, backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            if !DISC.is_null() {
                cdio_destroy(DISC);
                DISC = std::ptr::null_mut();
            }
        }

        INITIALIZED = false;
    }
}

pub(crate) fn change_disc(_config: &Config, backend_params: &BackendParams, path: &Path) -> Result<(), String> {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            if !DISC.is_null() {
                cdio_destroy(DISC);
                DISC = std::ptr::null_mut();
            }

            assert!(DISC.is_null());

            let cstr = CString::new(path.to_string_lossy().as_ref().to_owned()).unwrap();

            info!("Changing disc to {}", path.display());
            DISC = cdio_open(cstr.as_bytes_with_nul().as_ptr() as *const i8, driver_id_t_DRIVER_UNKNOWN);

            if DISC.is_null() {
                Err("Changing disc failed: NULL disc returned; check it's supported by libcdio".to_owned())
            } else {
                Ok(())
            }
        } else {
            Err("Not initialized".to_owned())
        }
    }
}
