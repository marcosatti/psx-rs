use crate::system::types::State as SystemState;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::b8_memory_mapper::*;
use crate::types::register::b32_register::B32Register;
use std::sync::atomic::{AtomicBool, Ordering};
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

pub struct State {
    pub timer0_count: B32Register,
    pub timer0_mode: Mode,
    pub timer0_target: B32Register,
    pub timer0_state: TimerState,

    pub timer1_count: B32Register,
    pub timer1_mode: Mode,
    pub timer1_target: B32Register,
    pub timer1_state: TimerState,

    pub timer2_count: B32Register,
    pub timer2_mode: Mode,
    pub timer2_target: B32Register,
    pub timer2_state: TimerState,
}

impl State {
    pub fn new() -> State {
        State {
            timer0_count: B32Register::new(),
            timer0_mode: Mode::new(),
            timer0_target: B32Register::new(),
            timer0_state: TimerState::new(),
            timer1_count: B32Register::new(),
            timer1_mode: Mode::new(),
            timer1_target: B32Register::new(),
            timer1_state: TimerState::new(),
            timer2_count: B32Register::new(),
            timer2_mode: Mode::new(),
            timer2_target: B32Register::new(),
            timer2_state: TimerState::new(),
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
}

impl B8MemoryMap for Mode {
    fn read_u16(&mut self, offset: u32) -> ReadResult<u16> {
        let result = B8MemoryMap::read_u16(&mut self.register, offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    fn write_u16(&mut self, offset: u32, value: u16) -> WriteResult {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        //assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u16(&mut self.register, offset, value)
    }

    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        let result = B8MemoryMap::read_u32(&mut self.register, offset);
        self.read_latch.store(true, Ordering::Release);
        result
    }

    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        // BIOS writes consecutively to this register without a chance to acknowledge...
        //assert!(!self.write_latch.load(Ordering::Acquire), "Write latch still on");
        self.write_latch.store(true, Ordering::Release);
        B8MemoryMap::write_u32(&mut self.register, offset, value)
    }
}

pub fn initialize(state: &mut SystemState) {
    state.r3000.memory_mapper.map(
        0x1F80_1100,
        4,
        &mut state.timers.timer0_count as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1104,
        4,
        &mut state.timers.timer0_mode as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1108,
        4,
        &mut state.timers.timer0_target as *mut dyn B8MemoryMap,
    );

    state.r3000.memory_mapper.map(
        0x1F80_1110,
        4,
        &mut state.timers.timer1_count as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1114,
        4,
        &mut state.timers.timer1_mode as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1118,
        4,
        &mut state.timers.timer1_target as *mut dyn B8MemoryMap,
    );

    state.r3000.memory_mapper.map(
        0x1F80_1120,
        4,
        &mut state.timers.timer2_count as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1124,
        4,
        &mut state.timers.timer2_mode as *mut dyn B8MemoryMap,
    );
    state.r3000.memory_mapper.map(
        0x1F80_1128,
        4,
        &mut state.timers.timer2_target as *mut dyn B8MemoryMap,
    );
}
