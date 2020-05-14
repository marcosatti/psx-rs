use crate::{
    system::{
        padmc::constants::*,
        padmc::types::*,
        types::State,
    },
    types::memory::LatchKind,
};
use crate::utilities::bool_to_flag;

pub fn handle_stat(state: &State) {
    state.padmc.stat.acknowledge(|_value, latch_kind| {
        match latch_kind {
            LatchKind::Read => calculate_stat_value(state),
            _ => panic!("Write to STAT register!"),
        }
    });
}

pub fn handle_ctrl(state: &State, controller_state: &mut ControllerState) {
    state.padmc.ctrl.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Write => {
                controller_state.tx_enabled = CTRL_TXEN.extract_from(value) > 0;

                controller_state.joy_select_enabled = CTRL_JOYN_OUTPUT.extract_from(value) > 0; 
                
                controller_state.ack_interrupt_enabled = CTRL_ACKINT_ENABLE.extract_from(value) > 0; 
                
                controller_state.use_joy2 = CTRL_JOY_SLOT.extract_from(value) > 0; 

                if CTRL_ACK.extract_from(value) > 0 {
                    //log::debug!("ACK bit acknowledged");
                }

                if CTRL_RESET.extract_from(value) > 0 {
                    //log::debug!("RESET bit acknowledged")
                }

                calculate_ctrl_value(controller_state)
            }
            LatchKind::Read => value,
        }
    });
}

fn calculate_stat_value(state: &State) -> u32 {
    let mut value = 0;

    value = STAT_TXRDY_1.insert_into(value, 1);
    value = STAT_RXFIFO_READY.insert_into(value, bool_to_flag(state.padmc.rx_fifo.is_empty()));
    value = STAT_TXRDY_2.insert_into(value, 1);

    // Baudrate timer not implemented.

    value
}

fn calculate_ctrl_value(controller_state: &mut ControllerState) -> u16 {
    let mut value = 0;

    value = CTRL_TXEN.insert_into(value, bool_to_flag(controller_state.tx_enabled) as u16);
    value = CTRL_JOYN_OUTPUT.insert_into(value, bool_to_flag(controller_state.joy_select_enabled) as u16);
    value = CTRL_ACKINT_ENABLE.insert_into(value, bool_to_flag(controller_state.ack_interrupt_enabled) as u16);
    value = CTRL_JOY_SLOT.insert_into(value, bool_to_flag(controller_state.use_joy2) as u16);

    value
}
