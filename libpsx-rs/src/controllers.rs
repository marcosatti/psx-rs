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

pub struct ControllerState<'a: 'b, 'b: 'c, 'c> {
    resources: &'c mut Resources,
    video_backend: &'c VideoBackend<'a, 'b>,
    audio_backend: &'c AudioBackend<'a, 'b>,
    cdrom_backend: &'c CdromBackend<'a, 'b>,
}

impl<'a: 'b, 'b: 'c, 'c> ControllerState<'a, 'b, 'c> {
    pub unsafe fn from_core_state(state: &State<'a, 'b, 'c>) -> ControllerState<'a, 'b, 'c> {
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
