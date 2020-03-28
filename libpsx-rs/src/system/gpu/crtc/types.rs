use std::time::Duration;

pub struct Crtc {
    pub frame_elapsed: Duration,
    pub scanline_elapsed: Duration,
}

impl Crtc {
    pub fn new() -> Crtc {
        Crtc {
            frame_elapsed: Duration::from_secs(0),
            scanline_elapsed: Duration::from_secs(0),
        }
    }
}
