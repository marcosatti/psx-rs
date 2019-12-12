use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum ClockSource {
    System,
    Dotclock,
    Hblank,
    System8,
}

pub struct TimerState {
    pub clock_source: ClockSource,
    pub current_elapsed: Duration,
    pub acknowledged_elapsed: Duration,
}

impl TimerState {
    pub fn new() -> TimerState {
        TimerState {
            clock_source: ClockSource::System,
            current_elapsed: Duration::from_secs(0),
            acknowledged_elapsed: Duration::from_secs(0),
        }
    }
}
