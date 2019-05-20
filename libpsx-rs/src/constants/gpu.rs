pub mod crtc;

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6; // Varies depending on NTSC/PAL. Due to the way the controller logic works this isnt even accurate, so this value is just a way to keep relative timing correct.

pub const VRAM_WIDTH_16B: usize = 1024; // Width in terms of halfwords (16 bit).
pub const VRAM_HEIGHT_LINES: usize = 512;

pub const _TEXPAGE_WIDTH: usize = 256;
pub const _TEXPAGE_HEIGHT: usize = 256;
