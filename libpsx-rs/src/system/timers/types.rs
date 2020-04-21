use crate::types::memory::b8_memory::B8Memory;
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
    pub timer0_count: B8Memory,
    pub timer0_mode: Mode,
    pub timer0_target: B8Memory,

    pub timer1_count: B8Memory,
    pub timer1_mode: Mode,
    pub timer1_target: B8Memory,

    pub timer2_count: B8Memory,
    pub timer2_mode: Mode,
    pub timer2_target: B8Memory,

    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            timer0_count: B8Memory::new(4),
            timer0_mode: Mode::new(),
            timer0_target: B8Memory::new(4),
            timer1_count: B8Memory::new(4),
            timer1_mode: Mode::new(),
            timer1_target: B8Memory::new(4),
            timer2_count: B8Memory::new(4),
            timer2_mode: Mode::new(),
            timer2_target: B8Memory::new(4),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}

pub struct Mode {
    pub register: B8Memory,
    pub write_latch: AtomicBool,
    pub read_latch: AtomicBool,
}

impl Mode {
    pub fn new() -> Mode {
        Mode {
            register: B8Memory::new(4),
            write_latch: AtomicBool::new(false),
            read_latch: AtomicBool::new(false),
        }
    }

    pub fn read_u16(&self, offset: u32) -> u16 {
        let result = self.register.read_u16(offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    pub fn write_u16(&self, offset: u32, value: u16) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u16(offset, value);
    }

    pub fn read_u32(&self, offset: u32) -> u32 {
        let result = self.register.read_u32(offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    pub fn write_u32(&self, offset: u32, value: u32) {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        // assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        self.register.write_u32(offset, value);
    }
}
