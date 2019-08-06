pub mod crtc;

pub const CLOCK_SPEED_NTSC: f64 = 53.693175 * 1e6; 
pub const _CLOCK_SPEED_PAL: f64 = 53.203425 * 1e6; 

pub const VRAM_WIDTH_16B: usize = 1024; // Width in terms of halfwords (16 bit).
pub const VRAM_HEIGHT_LINES: usize = 512;

pub const _TEXPAGE_WIDTH: usize = 256;
pub const _TEXPAGE_HEIGHT: usize = 256;
