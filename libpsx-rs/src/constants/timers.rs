use std::time::Duration;

pub const CLOCK_SPEED: f64 = 33.8688 * 1e6;
pub const HBLANK_INTERVAL_NTSC: Duration = Duration::from_nanos(10900); // 10.9 us
pub const HBLANK_INTERVAL_PAL: Duration = Duration::from_nanos(12000); // 12.0 us
