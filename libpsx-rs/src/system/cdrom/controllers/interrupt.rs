use crate::system::{
    cdrom::{
        constants::*,
        types::*,
    },
    intc::types::Line,
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn handle_irq_raise(state: &State, controller_state: &mut ControllerState, interrupt_index: usize) -> ControllerResult<()> {
    if controller_state.interrupt_index > 0 {
        return Err("Previous interrupt hasn't been acknowledged".into());
    }

    if interrupt_index >= 8 || interrupt_index == 0 {
        return Err(format!("Invalid interrupt index trying to be set: {}", interrupt_index));
    }

    let interrupt_enable = &state.cdrom.interrupt_enable;
    let interrupt_enable_value = interrupt_enable.read_bitfield(INTERRUPT_FLAGS) as usize;

    if (interrupt_enable_value & interrupt_index) != interrupt_index {
        return Err("IRQ pending but corresponding enable flags not set - will never trigger!".into());
    }

    controller_state.interrupt_index = interrupt_index;
    state.cdrom.interrupt_flag.update::<_, String>(|v| Ok(INTERRUPT_FLAGS.insert_into(v, controller_state.interrupt_index as u8)))?;

    state.intc.stat.assert_line(Line::Cdrom);

    Ok(())
}
