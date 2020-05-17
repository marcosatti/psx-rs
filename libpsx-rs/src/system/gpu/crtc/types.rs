use parking_lot::Mutex;
use std::time::Duration;

pub(crate) struct ControllerState {
    pub(crate) frame_elapsed: Duration,
    pub(crate) scanline_elapsed: Duration,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            frame_elapsed: Duration::from_secs(0),
            scanline_elapsed: Duration::from_secs(0),
        }
    }
}

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
