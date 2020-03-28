use std::time::Duration;
use crate::types::bitfield::Bitfield;

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6;
pub const DOTCLOCK_320_INTERVAL_NTSC: Duration = Duration::from_nanos(150); // 150.312650313 ns per dot (6.6528 MHz)
pub const SCANLINE_INTERVAL_NTSC: Duration = Duration::from_nanos(63_600); // 63.6 us per scanline
pub const SCANLINE_INTERVAL_PAL: Duration = Duration::from_nanos(64_000); // 64.0 us per scanline
pub const HBLANK_INTERVAL_NTSC: Duration = Duration::from_nanos(10_900); // 10.9 us per hblank
pub const HBLANK_INTERVAL_PAL: Duration = Duration::from_nanos(12_000); // 12.0 us per hblank
pub const SYSTEM_CLOCK_INTERVAL: Duration = Duration::from_nanos(30); // 29.525699169 ns per tick (33.8688 MHz)
pub const SYSTEM_CLOCK_8_INTERVAL: Duration = Duration::from_nanos(236); // 236.205593348 ns per tick (33.8688/8 MHz)

pub const MODE_SYNC_EN: Bitfield = Bitfield::new(0, 1);
pub const MODE_SYNC_MODE: Bitfield = Bitfield::new(1, 2);
pub const MODE_RESET: Bitfield = Bitfield::new(3, 1);
pub const MODE_IRQ_TARGET: Bitfield = Bitfield::new(4, 1);
pub const MODE_IRQ_OVERFLOW: Bitfield = Bitfield::new(5, 1);
pub const MODE_IRQ_REPEAT: Bitfield = Bitfield::new(6, 1);
pub const MODE_IRQ_PULSE: Bitfield = Bitfield::new(7, 1);
pub const MODE_CLK_SRC: Bitfield = Bitfield::new(8, 2);
pub const MODE_IRQ_STATUS: Bitfield = Bitfield::new(10, 1);
pub const MODE_TARGET_HIT: Bitfield = Bitfield::new(11, 1);
pub const MODE_OVERFLOW_HIT: Bitfield = Bitfield::new(12, 1);
