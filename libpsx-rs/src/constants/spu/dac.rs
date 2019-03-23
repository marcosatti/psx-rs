use std::time::Duration;

pub const SAMPLE_RATE_PERIOD: Duration = Duration::from_nanos(22676); // 1 / 44100th of a second
pub const BUFFER_SIZE: usize = 2048;
