use crate::types::bitfield::Bitfield;

pub const CLOCK_SPEED_NTSC: f64 = 53.693175 * 1e6; 
pub const _CLOCK_SPEED_PAL: f64 = 53.203425 * 1e6; 
pub const VRAM_WIDTH_16B: usize = 1024; // Width in terms of halfwords (16 bit).
pub const VRAM_HEIGHT_LINES: usize = 512;
pub const _TEXPAGE_WIDTH: usize = 256;
pub const _TEXPAGE_HEIGHT: usize = 256;


pub const GP_CMD: Bitfield = Bitfield::new(24, 8);
pub const STAT_TEXPAGEX: Bitfield = Bitfield::new(0, 4);
pub const STAT_TEXPAGEY: Bitfield = Bitfield::new(4, 1);
pub const STAT_TRANSPARENCY: Bitfield = Bitfield::new(5, 2);
pub const STAT_TEXPAGE_COLORS: Bitfield = Bitfield::new(7, 2);
pub const STAT_DITHER: Bitfield = Bitfield::new(9, 1);
pub const STAT_DRAW_DISPLAY: Bitfield = Bitfield::new(10, 1);
pub const STAT_DRAW_MASK: Bitfield = Bitfield::new(11, 1);
pub const STAT_DRAW_PIXELS: Bitfield = Bitfield::new(12, 1);
pub const _STAT_INTERLACE_FIELD: Bitfield = Bitfield::new(13, 1);
pub const STAT_REVERSE: Bitfield = Bitfield::new(14, 1);
pub const STAT_TEXTURE_DISABLE: Bitfield = Bitfield::new(15, 1);
pub const STAT_HORIZONTAL_RES_2: Bitfield = Bitfield::new(16, 1);
pub const STAT_HORIZONTAL_RES_1: Bitfield = Bitfield::new(17, 2);
pub const STAT_VERTICAL_RES: Bitfield = Bitfield::new(19, 1);
pub const STAT_VIDEO_MODE: Bitfield = Bitfield::new(20, 1);
pub const STAT_DISPLAY_COLOR_DEPTH: Bitfield = Bitfield::new(21, 1);
pub const STAT_INTERLACE_VERTICAL: Bitfield = Bitfield::new(22, 1);
pub const STAT_DISPLAY_ENABLE: Bitfield = Bitfield::new(23, 1);
pub const STAT_IRQ_REQUEST: Bitfield = Bitfield::new(24, 1);
pub const STAT_DMA_REQUEST: Bitfield = Bitfield::new(25, 1);
pub const STAT_RECV_CMD: Bitfield = Bitfield::new(26, 1);
pub const STAT_SEND_VRAM: Bitfield = Bitfield::new(27, 1);
pub const STAT_RECV_DMA: Bitfield = Bitfield::new(28, 1);
pub const STAT_DMA_DIRECTION: Bitfield = Bitfield::new(29, 2);
pub const STAT_DRAWING_ODD: Bitfield = Bitfield::new(31, 1);

