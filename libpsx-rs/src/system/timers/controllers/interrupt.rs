use crate::system::{
    intc::types::Line,
    timers::{
        controllers::{
            register::*,
            timer::*,
        },
        types::*,
    },
    types::{ControllerResult, State},
};

pub(crate) fn handle_irq_trigger(state: &State, timer_state: &mut TimerState, timer_id: usize, irq_type: IrqType) -> ControllerResult {
    // First check if we are in one-shot mode, don't raise an IRQ if we have already done so.
    if timer_state.oneshot_mode {
        if timer_state.irq_raised {
            return Ok(());
        }
    }

    let irq_condition_matches = match irq_type {
        IrqType::Overflow => timer_state.irq_on_overflow,
        IrqType::Target => timer_state.irq_on_target,
    };

    if irq_condition_matches {
        timer_state.irq_raised = true;
        handle_irq_raise(state, timer_state, timer_id)?;
    }

    Ok(())
}

fn handle_irq_raise(state: &State, timer_state: &mut TimerState, timer_id: usize) -> ControllerResult {
    let mode = get_mode(state, timer_id);

    let raise_irq = if timer_state.irq_toggle {
        timer_state.irq_raised ^= true;
        timer_state.irq_raised
    } else {
        // Pulse mode - don't need to do anything.
        true
    };

    mode.update(|_| Ok(calculate_mode_value(timer_state)))?;

    if raise_irq {
        state.intc.stat.assert_line(match timer_id {
            0 => Line::Tmr0,
            1 => Line::Tmr1,
            2 => Line::Tmr2,
            _ => unreachable!(),
        });
    }

    Ok(())
}
