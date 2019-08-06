use std::time::Duration;

pub const CLOCK_SPEED_NTSC: f64 = 3.58 * 1e6;
pub const _CLOCK_SPEED_PAL: f64 = 4.43 * 1e6;

pub const REFRESH_RATE_NTSC_PERIOD: Duration = Duration::from_nanos(16666667); // 1 / 60th of a second 
pub const _REFRESH_RATE_PAL_PERIOD: Duration = Duration::from_nanos(20000000); // 1 / 50th of a second 
