//use libmirage_sys::*;
use crate::backends::cdrom::libmirage::*;

pub fn disc_mode(backend_params: &BackendParams) -> usize {
    let (_context_guard, _context) = backend_params.context.guard();

    unimplemented!();
}
