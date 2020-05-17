use crate::backends::context::*;
use libmirage_sys::*;
use log::info;
use std::{
    ffi::{
        CStr,
        CString,
    },
    path::Path,
};

static mut INITIALIZED: bool = false;

pub(crate) static mut DISC: *mut MirageDisc = std::ptr::null_mut();

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, *mut MirageContext>,
}

pub(crate) fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);

        DISC = std::ptr::null_mut();

        INITIALIZED = true;
    }
}

pub(crate) fn teardown(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            if !DISC.is_null() {
                g_clear_object((&mut DISC as *mut *mut MirageDisc) as *mut *mut GObject);
            }
        }

        INITIALIZED = false;
    }
}

pub(crate) fn change_disc(backend_params: &BackendParams, path: &Path) {
    let (_context_guard, context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            if !DISC.is_null() {
                g_clear_object((&mut DISC as *mut *mut MirageDisc) as *mut *mut GObject);
            }

            assert!(DISC.is_null());

            let cstr = CString::new(path.to_string_lossy().as_ref().to_owned()).unwrap();
            let buffer = [cstr.as_bytes_with_nul().as_ptr(), std::ptr::null_mut()];
            let mut error: *mut GError = std::ptr::null_mut();

            info!("Changing disc to {}", path.display());
            DISC = mirage_context_load_image(*context, buffer.as_ptr() as *mut *mut i8, &mut error as *mut *mut GError);

            if DISC.is_null() {
                assert!(!error.is_null());
                let error_cstr = CStr::from_ptr((*error).message).to_string_lossy();
                panic!("Changing disc failed: {}", error_cstr.as_ref());
            } else {
                assert!(error.is_null());
            }
        }
    }
}
