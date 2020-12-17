use crate::types::exclusive_state::ExclusiveState;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    pub(crate) scanline_clock: f32,
    pub(crate) frame_clock: f32,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            scanline_clock: 0.0,
            frame_clock: 0.0,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct Crtc {
    pub(crate) controller_state: ExclusiveState<ControllerState>,
}

impl Crtc {
    pub(crate) fn new() -> Crtc {
        Crtc {
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
