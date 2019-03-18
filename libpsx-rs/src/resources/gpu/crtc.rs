use std::time::Duration;

pub struct Crtc {
    pub vblank_time: Duration,
    pub drawing_odd: bool,
}

impl Crtc {
    pub fn new() -> Crtc {
        Crtc {
            vblank_time: Duration::from_nanos(0),
            drawing_odd: false,
        }
    }
}
