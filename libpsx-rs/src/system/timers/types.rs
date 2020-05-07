use crate::types::memory::*;
use parking_lot::Mutex;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum IrqType {
    Overflow,
    Target,
}

#[derive(Copy, Clone, Debug)]
pub enum ClockSource {
    System,
    Dotclock,
    Hblank,
    System8,
}

pub struct TimerState {
    pub reset_on_target: bool,
    pub irq_on_target: bool,
    pub irq_on_overflow: bool,
    pub oneshot_mode: bool,
    pub irq_toggle: bool,
    pub clock_source: ClockSource,

    pub current_elapsed: Duration,
    pub acknowledged_elapsed: Duration,
    pub irq_raised: bool,
}

impl TimerState {
    pub fn new() -> TimerState {
        TimerState {
            clock_source: ClockSource::System,
            current_elapsed: Duration::from_secs(0),
            acknowledged_elapsed: Duration::from_secs(0),
            reset_on_target: false,
            irq_raised: false,
            irq_on_target: false,
            irq_on_overflow: false,
            irq_toggle: false,
            oneshot_mode: false,
        }
    }
}

pub struct ControllerState {
    pub timer0_state: TimerState,
    pub timer1_state: TimerState,
    pub timer2_state: TimerState,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
            timer0_state: TimerState::new(),
            timer1_state: TimerState::new(),
            timer2_state: TimerState::new(),
        }
    }
}

pub struct State {
    pub timer0_count: B32LevelRegister,
    pub timer0_mode: B32EdgeRegister,
    pub timer0_target: B32LevelRegister,

    pub timer1_count: B32LevelRegister,
    pub timer1_mode: B32EdgeRegister,
    pub timer1_target: B32LevelRegister,

    pub timer2_count: B32LevelRegister,
    pub timer2_mode: B32EdgeRegister,
    pub timer2_target: B32LevelRegister,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            timer0_count: B32LevelRegister::new(),
            timer0_mode: B32EdgeRegister::new(),
            timer0_target: B32LevelRegister::new(),
            timer1_count: B32LevelRegister::new(),
            timer1_mode: B32EdgeRegister::new(),
            timer1_target: B32LevelRegister::new(),
            timer2_count: B32LevelRegister::new(),
            timer2_mode: B32EdgeRegister::new(),
            timer2_target: B32LevelRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
