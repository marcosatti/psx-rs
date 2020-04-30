use std::time::Duration;
use parking_lot::Mutex;

pub struct ControllerState {
    pub frame_elapsed: Duration,
    pub scanline_elapsed: Duration,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            frame_elapsed: Duration::from_secs(0),
            scanline_elapsed: Duration::from_secs(0),
        }
    }
}

pub struct Crtc {
    pub controller_state: Mutex<ControllerState>,
}

impl Crtc {
    pub fn new() -> Crtc {
        Crtc { 
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
