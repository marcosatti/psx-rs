pub mod r3000;
pub mod gpu;
pub mod intc;
pub mod dmac;
pub mod spu;
pub mod cdrom;
pub mod padmc;
pub mod timers;

use std::time::Duration;
use crate::State;
use crate::resources::Resources;
use crate::backends::video::VideoBackend;
use crate::backends::audio::AudioBackend;
use crate::backends::cdrom::CdromBackend;

pub struct ControllerState<'b, 'a: 'b> {
    resources: &'b mut Resources,
    video_backend: &'b VideoBackend<'a>,
    audio_backend: &'b AudioBackend<'a>,
    cdrom_backend: &'b CdromBackend<'a>,
}

impl<'b, 'a: 'b> ControllerState<'b, 'a> {
    pub unsafe fn from_core_state(state: &State<'b, 'a>) -> ControllerState<'b, 'a> {
        ControllerState {
            resources: state.resources.as_mut().unwrap(),
            video_backend: state.video_backend,
            audio_backend: state.audio_backend,
            cdrom_backend: state.cdrom_backend,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Time(Duration),
}
