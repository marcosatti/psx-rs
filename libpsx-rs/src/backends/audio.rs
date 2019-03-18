pub mod open_al;

use crate::backends::audio::open_al::*;

pub enum AudioBackend<'a> {
    OpenAl(BackendParams<'a>),
}
