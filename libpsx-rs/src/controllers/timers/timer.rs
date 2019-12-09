use crate::resources::Resources;
use crate::resources::timers::register::*;
use crate::types::register::b32_register::B32Register;

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
