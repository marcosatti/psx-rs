use crate::system::{
    intc::types::Line,
    timers::{
        controllers::{
            register::*,
            timer::*,
        },
        types::*,
    },
    types::State,
};

pub(crate) fn handle_irq_trigger(state: &State, controller_state: &mut ControllerState, timer_id: usize, irq_type: IrqType) {
    let timer_state = get_state(controller_state, timer_id);

    // First check if we are in one-shot mode, don't raise an IRQ if we have already done so.
    if timer_state.oneshot_mode {
        if timer_state.irq_raised {
            return;
        }
    }

    let irq_condition_matches = match irq_type {
        IrqType::Overflow => timer_state.irq_on_overflow,
        IrqType::Target => timer_state.irq_on_target,
    };

    if irq_condition_matches {
        timer_state.irq_raised = true;
        handle_irq_raise(state, controller_state, timer_id);
    }
}

fn handle_irq_raise(state: &State, controller_state: &mut ControllerState, timer_id: usize) {
    let mode = get_mode(state, timer_id);
    let timer_state = get_state(controller_state, timer_id);

    let raise_irq = if timer_state.irq_toggle {
        timer_state.irq_raised ^= true;
        timer_state.irq_raised
    } else {
        // Pulse mode - don't need to do anything.
        true
    };

    mode.update(|_| calculate_mode_value(timer_state));

    if raise_irq {
        state.intc.stat.assert_line(match timer_id {
            0 => Line::Tmr0,
            1 => Line::Tmr1,
            2 => Line::Tmr2,
            _ => unreachable!(),
        });
    }
}
