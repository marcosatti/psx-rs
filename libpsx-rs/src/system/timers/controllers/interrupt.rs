use crate::system::{
    timers::{
        constants::*,
        controllers::timer::*,
        types::*,
    },
    types::State,
};
use crate::system::intc::types::Line;

pub fn handle_irq_trigger(state: &State, controller_state: &mut ControllerState, timer_id: usize, irq_type: IrqType) {
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
        let mut bit = 0;

        mode.update(|value| {
            bit = MODE_IRQ_STATUS.extract_from(value) ^ 1;
            MODE_IRQ_STATUS.insert_into(value, bit)
        });

        bit == 0
    } else {
        // Pulse mode.
        // TODO: Do nothing? How long is a few clock cycles? Will the BIOS see this? Probably not...
        log::warn!("Pulse IRQ mode not implemented properly?"); 
        true
    };

    if raise_irq {
        state.intc.stat.assert_line(match timer_id {
            0 => Line::Tmr0,
            1 => Line::Tmr1,
            2 => Line::Tmr2,
            _ => unreachable!(),
        });
        log::debug!("Raised INTC IRQ for timer {}", timer_id);
    }
}
