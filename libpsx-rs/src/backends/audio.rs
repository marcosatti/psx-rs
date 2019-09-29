pub mod openal;

use crate::backends::audio::openal::*;

pub enum AudioBackend<'a> {
    None,
    Openal(BackendParams<'a>),
}
