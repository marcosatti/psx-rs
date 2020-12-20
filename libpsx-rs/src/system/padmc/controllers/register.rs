use crate::{
    system::{
        padmc::{
            constants::*,
            types::*,
        },
        types::{
            ControllerResult,
            State,
        },
    },
    types::memory::LatchKind,
};

pub(crate) fn handle_ctrl(state: &State, controller_state: &mut ControllerState) -> ControllerResult<()> {
    state.padmc.ctrl.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => Ok(value),
            LatchKind::Write => {
                controller_state.tx_enabled = CTRL_TXEN.extract_from(value) > 0;

                controller_state.joy_select_enabled = CTRL_JOYN_OUTPUT.extract_from(value) > 0;

                controller_state.ack_interrupt_enabled = CTRL_ACKINT_ENABLE.extract_from(value) > 0;

                controller_state.use_joy2 = CTRL_JOY_SLOT.extract_from(value) > 0;

                if CTRL_ACK.extract_from(value) > 0 {
                    // log::debug!("ACK bit acknowledged");
                }

                if CTRL_RESET.extract_from(value) > 0 {
                    // log::debug!("RESET bit acknowledged")
                }

                Ok(value)
            },
        }
    })
}
