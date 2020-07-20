use crate::system::{
    dmac::{
        constants::*,
        controllers::{
            channel::*,
            register::*,
        },
        types::*,
    },
    intc::types::Line,
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn handle_irq_trigger(transfer_state: &mut TransferState) {
    if transfer_state.interrupt_enabled {
        transfer_state.interrupted = true;
    }
}

pub(crate) fn handle_irq_raise(state: &State, controller_state: &mut ControllerState) -> ControllerResult<()> {
    // TODO: Force IRQ bit not handled yet.

    let mut master_trigger = false;
    if controller_state.master_interrupt_enabled {
        for channel_id in 0..CHANNEL_COUNT {
            let transfer_state = get_transfer_state(controller_state, channel_id);
            master_trigger |= transfer_state.interrupt_enabled && transfer_state.interrupted;
        }
    }

    let mut raise_irq = false;
    if master_trigger {
        if !controller_state.master_interrupted {
            controller_state.master_interrupted = true;
            raise_irq = true;
        }
    } else {
        controller_state.master_interrupted = false;
    }

    state.dmac.dicr.update(|_| calculate_dicr_value(controller_state))?;

    if raise_irq {
        state.intc.stat.assert_line(Line::Dma);
    }

    Ok(())
}
