pub mod crtc;

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6; // Varies depending on NTSC/PAL. Due to the way the controller logic works this isnt even accurate, so this value is just a way to keep relative timing correct.
