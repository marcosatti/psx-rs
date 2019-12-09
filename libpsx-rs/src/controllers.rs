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
use crate::video::VideoBackend;
use crate::audio::AudioBackend;

pub struct ControllerState<'b, 'a: 'b> {
    resources: &'b mut Resources,
    video_backend: &'b VideoBackend<'a>,
    audio_backend: &'b AudioBackend<'a>,
}

impl<'b, 'a: 'b> ControllerState<'b, 'a> {
    pub unsafe fn from_core_state(state: &State<'b, 'a>) -> ControllerState<'b, 'a> {
        ControllerState {
            resources: state.resources.as_mut().unwrap(),
            video_backend: state.video_backend,
            audio_backend: state.audio_backend,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Time(Duration),
}
