use openal_sys::*;
use crate::backends::context::*;

pub struct BackendParams<'a> {
    pub context: BackendContext<'a, *mut ALCcontext_struct>,
}
