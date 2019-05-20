use std::time::Duration;

pub const _SAMPLE_RATE: usize = 44100;
pub const SAMPLE_RATE_PERIOD: Duration = Duration::from_nanos(22676); // 1 / 44100th of a second
pub const BUFFER_SIZE: usize = 2048;
pub const VOICES_COUNT: usize = 24;
