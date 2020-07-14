use crate::types::memory::*;
use crate::types::exclusive_state::ExclusiveState;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum IrqType {
    Overflow,
    Target,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum ClockSource {
    System,
    Dotclock,
    Hblank,
    System8,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct TimerState {
    pub(crate) clock: f64,
    pub(crate) reset_on_target: bool,
    pub(crate) irq_on_target: bool,
    pub(crate) irq_on_overflow: bool,
    pub(crate) oneshot_mode: bool,
    pub(crate) irq_toggle: bool,
    pub(crate) clock_source: ClockSource,
    pub(crate) clock_source_raw: u32,
    pub(crate) irq_raised: bool,
    pub(crate) target_hit: bool,
    pub(crate) overflow_hit: bool,
}

impl TimerState {
    pub(crate) fn new() -> TimerState {
        TimerState {
            clock: 0.0,
            clock_source: ClockSource::System,
            clock_source_raw: 0,
            reset_on_target: false,
            irq_raised: false,
            irq_on_target: false,
            irq_on_overflow: false,
            irq_toggle: false,
            oneshot_mode: false,
            target_hit: false,
            overflow_hit: false,
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    pub(crate) timer0_state: TimerState,
    pub(crate) timer1_state: TimerState,
    pub(crate) timer2_state: TimerState,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            timer0_state: TimerState::new(),
            timer1_state: TimerState::new(),
            timer2_state: TimerState::new(),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) timer0_count: B32LevelRegister,
    pub(crate) timer0_mode: B32EdgeRegister,
    pub(crate) timer0_target: B32LevelRegister,

    pub(crate) timer1_count: B32LevelRegister,
    pub(crate) timer1_mode: B32EdgeRegister,
    pub(crate) timer1_target: B32LevelRegister,

    pub(crate) timer2_count: B32LevelRegister,
    pub(crate) timer2_mode: B32EdgeRegister,
    pub(crate) timer2_target: B32LevelRegister,

    pub(crate) controller_state: ExclusiveState<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
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
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
