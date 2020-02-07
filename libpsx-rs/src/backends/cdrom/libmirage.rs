use libmirage_sys::*;
use crate::backends::context::*;

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, *mut MirageContext>,
}

pub fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();
}
