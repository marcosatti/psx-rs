pub mod openal;

use crate::backends::audio::openal::*;

pub enum AudioBackend<'a> {
    Openal(BackendParams<'a>),
}
