#![allow(dead_code)]

use libpsx_rs::backends::cdrom::*;

#[derive(Copy, Clone, Debug)]
pub(crate) enum CdromBackendKind {
    None,
    Libmirage,
    Libcdio,
}

pub(crate) fn initialize_cdrom_backend<'a>(kind: CdromBackendKind) -> CdromBackend<'a> {
    match kind {
        CdromBackendKind::None => CdromBackend::None,
        CdromBackendKind::Libmirage => initialize_cdrom_backend_libmirage(),
        CdromBackendKind::Libcdio => initialize_cdrom_backend_libcdio(),
    }
}

pub(crate) fn terminate_cdrom_backend(kind: CdromBackendKind) {
    match kind {
        CdromBackendKind::None => {},
        CdromBackendKind::Libmirage => terminate_cdrom_backend_libmirage(),
        CdromBackendKind::Libcdio => terminate_cdrom_backend_libcdio(),
    }
}

/// Libmirage

#[cfg(libmirage)]
static mut LIBMIRAGE_CONTEXT: *mut libmirage_sys::MirageContext = std::ptr::null_mut();

#[cfg(not(libmirage))]
pub(crate) fn initialize_cdrom_backend_libmirage<'a>() -> CdromBackend<'a> {
    panic!("Not available");
}

#[cfg(libmirage)]
pub(crate) fn initialize_cdrom_backend_libmirage<'a>() -> CdromBackend<'a> {
    use libmirage_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    unsafe {
        mirage_initialize(std::ptr::null_mut());
        LIBMIRAGE_CONTEXT = g_object_new(mirage_context_get_type(), std::ptr::null()) as *mut MirageContext;
        assert!(!LIBMIRAGE_CONTEXT.is_null());
    }

    let libmirage_version_string = unsafe { std::ffi::CStr::from_ptr(mirage_version_long).to_string_lossy().into_owned() };
    log::info!("CDROM initialized: libmirage {}", libmirage_version_string);

    CdromBackend::Libmirage(libmirage::BackendParams {
        context: BackendContext::new(&move || unsafe { LIBMIRAGE_CONTEXT }, &move || {}),
    })
}

#[cfg(not(libmirage))]
pub(crate) fn terminate_cdrom_backend_libmirage() {
    panic!("Not available");
}

#[cfg(libmirage)]
pub(crate) fn terminate_cdrom_backend_libmirage() {
    use libmirage_sys::*;

    unsafe {
        assert!(!LIBMIRAGE_CONTEXT.is_null());
        g_clear_object((&mut LIBMIRAGE_CONTEXT as *mut *mut MirageContext) as *mut *mut GObject);
    }
}

/// Libcdio

#[cfg(not(libcdio))]
pub(crate) fn initialize_cdrom_backend_libcdio<'a>() -> CdromBackend<'a> {
    panic!("Not available");
}

#[cfg(libcdio)]
pub(crate) fn initialize_cdrom_backend_libcdio<'a>() -> CdromBackend<'a> {
    use libcdio_sys::*;
    use libpsx_rs::backends::context::BackendContext;

    unsafe {
        let result = cdio_init();
        assert_eq!(result, 1);
    }

    let libcdio_version_string = unsafe { std::ffi::CStr::from_ptr(cdio_version_string).to_string_lossy().into_owned() };
    log::info!("CDROM initialized: libcdio {}", libcdio_version_string);

    CdromBackend::Libcdio(libcdio::BackendParams {
        context: BackendContext::new(&move || {}, &move || {}),
    })
}

#[cfg(not(libcdio))]
pub(crate) fn terminate_cdrom_backend_libcdio() {
    panic!("Not available");
}

#[cfg(libcdio)]
pub(crate) fn terminate_cdrom_backend_libcdio() {
}
