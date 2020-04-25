use crate::types::memory::*;
use std::{
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    time::Duration,
};
use parking_lot::Mutex;

#[derive(Copy, Clone, Debug)]
pub enum IrqType {
    None,
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
            irq_raised: false,
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
    pub timer0_count: B32Register,
    pub timer0_mode: Mode,
    pub timer0_target: B32Register,

    pub timer1_count: B32Register,
    pub timer1_mode: Mode,
    pub timer1_target: B32Register,

    pub timer2_count: B32Register,
    pub timer2_mode: Mode,
    pub timer2_target: B32Register,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            timer0_count: B32Register::new(),
            timer0_mode: Mode::new(),
            timer0_target: B32Register::new(),
            timer1_count: B32Register::new(),
            timer1_mode: Mode::new(),
            timer1_target: B32Register::new(),
            timer2_count: B32Register::new(),
            timer2_mode: Mode::new(),
            timer2_target: B32Register::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

pub struct Mode {
    pub register: B32Register,
    pub write_latch: AtomicBool,
    pub read_latch: AtomicBool,
}

impl Mode {
    pub fn new() -> Mode {
        Mode {
            register: B32Register::new(),
            write_latch: AtomicBool::new(false),
            read_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u16(&self) -> u16 {
        let result = self.register.read_u16();
        self.read_latch.store(true, Ordering::Release);
        result
    }

    pub fn write_u16(&self, value: u16) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u16(value);
    }

    pub fn read_u32(&self) -> u32 {
        let result = self.register.read_u32();
        self.read_latch.store(true, Ordering::Release);
        result
    }

    pub fn write_u32(&self, value: u32) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u32(value);
    }
}
