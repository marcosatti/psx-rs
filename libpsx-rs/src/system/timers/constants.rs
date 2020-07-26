use crate::types::bitfield::Bitfield;

pub(crate) const _CLOCK_SPEED: f64 = 33.8688 * 1e6;
pub(crate) const DOTCLOCK_320_PERIOD_NTSC: f64 = 150.0 * 1e-9; // 150.312650313 ns per dot (6.6528 MHz)
pub(crate) const SCANLINE_PERIOD_NTSC: f64 = 63.6 * 1e-6; // 63.6 us per scanline
pub(crate) const _SCANLINE_PERIOD_PAL: f64 = 64.0 * 1e-6; // 64.0 us per scanline
pub(crate) const _HBLANK_PERIOD_NTSC: f64 = 10.9 * 1e-6; // 10.9 us per hblank
pub(crate) const _HBLANK_PERIOD_PAL: f64 = 12.0 * 1e-6; // 12.0 us per hblank
pub(crate) const SYSTEM_CLOCK_PERIOD: f64 = 1.0 / (33.8688 * 1e6); // 29.525699169 ns per tick (33.8688 MHz)
pub(crate) const SYSTEM_CLOCK_8_PERIOD: f64 = SYSTEM_CLOCK_PERIOD * 8.0; // 236.205593348 ns per tick (33.8688/8 MHz)
pub(crate) const TIMER_COUNT: usize = 3;

pub(crate) const MODE_SYNC_ENABLE: Bitfield = Bitfield::new(0, 1);
pub(crate) const MODE_SYNC_MODE: Bitfield = Bitfield::new(1, 2);
pub(crate) const MODE_SYNC_ENABLE_MODE: Bitfield = Bitfield::new(0, 3);
pub(crate) const MODE_RESET: Bitfield = Bitfield::new(3, 1);
pub(crate) const MODE_IRQ_TARGET: Bitfield = Bitfield::new(4, 1);
pub(crate) const MODE_IRQ_OVERFLOW: Bitfield = Bitfield::new(5, 1);
pub(crate) const MODE_IRQ_REPEAT: Bitfield = Bitfield::new(6, 1);
pub(crate) const MODE_IRQ_PULSE: Bitfield = Bitfield::new(7, 1);
pub(crate) const MODE_CLK_SRC: Bitfield = Bitfield::new(8, 2);
pub(crate) const MODE_IRQ_STATUS: Bitfield = Bitfield::new(10, 1);
pub(crate) const MODE_TARGET_HIT: Bitfield = Bitfield::new(11, 1);
pub(crate) const MODE_OVERFLOW_HIT: Bitfield = Bitfield::new(12, 1);
