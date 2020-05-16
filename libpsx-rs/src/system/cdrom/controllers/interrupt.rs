use crate::system::{
    cdrom::{
        constants::*,
        types::*,
    },
    intc::types::Line,
    types::State,
};

pub fn handle_irq_raise(state: &State, controller_state: &mut ControllerState, interrupt_index: usize) {
    assert_eq!(controller_state.interrupt_index, 0);
    assert!(interrupt_index < 16);
    assert_ne!(interrupt_index, 0);

    let interrupt_enable = &state.cdrom.interrupt_enable;
    let interrupt_enable_value = interrupt_enable.read_bitfield(INTERRUPT_FLAGS) as usize;

    if (interrupt_enable_value & interrupt_index) != interrupt_index {
        panic!("IRQ pending but corresponding enable flags not set - will never trigger!");
    }

    controller_state.interrupt_index = interrupt_index;
    state.cdrom.interrupt_flag.update(|_| calculate_interrupt_flag_value(controller_state));

    // log::debug!("Raised interrupt with index {}", interrupt_index);
    state.intc.stat.assert_line(Line::Cdrom);
}

pub fn calculate_interrupt_flag_value(controller_state: &ControllerState) -> u8 {
    let mut value = 0xFF;

    assert!(controller_state.interrupt_index < 16);
    value = INTERRUPT_FLAGS.insert_into(value, controller_state.interrupt_index as u8);

    value
}
