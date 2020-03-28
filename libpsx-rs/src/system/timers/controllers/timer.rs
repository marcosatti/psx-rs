use std::time::Duration;
use crate::system::Resources;
use crate::system::timers::register::*;
use crate::system::timers::timer::*;
use crate::types::register::b32_register::B32Register;

#[derive(Copy, Clone, Debug)]
pub enum IrqType {
    None,
    Overflow,
    Target,
}

pub fn get_count<'a, 'b>(resources: &'a mut Resources, timer_id: usize) -> &'b mut B32Register {
    let count = match timer_id {
        0 => &mut resources.timers.timer0_count,
        1 => &mut resources.timers.timer1_count,
        2 => &mut resources.timers.timer2_count,
        _ => unreachable!("Invalid timer ID"),
    };

    unsafe {
        (count as *mut B32Register).as_mut().unwrap()
    }
}

pub fn get_mode<'a, 'b>(resources: &'a mut Resources, timer_id: usize) -> &'b mut Mode {
    let mode = match timer_id {
        0 => &mut resources.timers.timer0_mode,
        1 => &mut resources.timers.timer1_mode,
        2 => &mut resources.timers.timer2_mode,
        _ => unreachable!("Invalid timer ID"),
    };

    unsafe {
        (mode as *mut Mode).as_mut().unwrap()
    }
}

pub fn get_target<'a, 'b>(resources: &'a mut Resources, timer_id: usize) -> &'b mut B32Register {
    let target = match timer_id {
        0 => &mut resources.timers.timer0_target,
        1 => &mut resources.timers.timer1_target,
        2 => &mut resources.timers.timer2_target,
        _ => unreachable!("Invalid timer ID"),
    };

    unsafe {
        (target as *mut B32Register).as_mut().unwrap()
    }
}

pub fn get_state<'a, 'b>(resources: &'a mut Resources, timer_id: usize) -> &'b mut TimerState {
    let state = match timer_id {
        0 => &mut resources.timers.timer0_state,
        1 => &mut resources.timers.timer1_state,
        2 => &mut resources.timers.timer2_state,
        _ => unreachable!("Invalid timer ID"),
    };

    unsafe {
        (state as *mut TimerState).as_mut().unwrap()
    }
}

pub fn handle_duration_clear(resources: &mut Resources, timer_id: usize) {
    let state = get_state(resources, timer_id);
    state.current_elapsed = Duration::from_secs(0);
    state.acknowledged_elapsed = Duration::from_secs(0);    
}
