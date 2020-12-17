use crate::system::{
    dmac::{
        constants::*,
        controllers::{
            channel::*,
        },
        types::*,
    },
    intc::types::Line,
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn handle_irq_trigger(state: &State, controller_state: &mut ControllerState, channel_id: usize) -> ControllerResult<()> {
    let raise_irq = {
        let transfer_state = get_transfer_state(controller_state, channel_id);
        if transfer_state.interrupt_enabled {
            transfer_state.interrupted = true;
            true
        } else {
            false
        }
    };

    if raise_irq {
        let raise_master_irq = controller_state.master_interrupt_enabled && (!controller_state.master_interrupted);

        state.dmac.dicr.update::<_, String>(|mut value| {
            value = DICR_IRQ_FLAG_BITFIELDS[channel_id].insert_into(value, 1);

            if raise_master_irq {
                value = DICR_IRQ_MASTER_FLAG.insert_into(value, 1);
            }

            Ok(value)
        })?;
        
        if raise_master_irq {
            controller_state.master_interrupted = true;
            state.intc.stat.assert_line(Line::Dma);
        }
    }

    Ok(())
}
