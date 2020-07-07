use parking_lot::Mutex;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct ControllerState {
    pub(crate) frame_elapsed: f64,
    pub(crate) scanline_elapsed: f64,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            frame_elapsed: 0.0,
            scanline_elapsed: 0.0,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) struct Crtc {
    pub(crate) controller_state: Mutex<ControllerState>,
}

impl Crtc {
    pub(crate) fn new() -> Crtc {
        Crtc {
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
