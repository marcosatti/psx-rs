use crate::{
    system::{
        cdrom::{
            constants::*,
            controllers::interrupt::*,
            types::*,
        },
        types::{
            ControllerResult,
            State,
        },
    },
    types::memory::*,
};

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState) -> ControllerResult {
    state.cdrom.command.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => Ok(value),
            LatchKind::Write => {
                if !controller_state.command_index.is_none() {
                    return Err("Write to command register before previous one is acknowledged".into());
                }

                controller_state.command_index = Some(value);
                Ok(value)
            },
        }
    })
}

pub(crate) fn handle_request(state: &State, controller_state: &mut ControllerState) -> ControllerResult {
    state.cdrom.request.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => Ok(value),
            LatchKind::Write => {
                if REQUEST_SMEN.extract_from(value) > 0 {
                    return Err("Command start interrupt not implemented".into());
                }

                if REQUEST_BFRD.extract_from(value) > 0 {
                    controller_state.load_data_flag = true;
                } else {
                    if !state.cdrom.data.is_empty() {
                        return Err("Data FIFO was not empty when a clear was requested".into());
                    }

                    state.cdrom.data.clear();
                }

                Ok(0)
            },
        }
    })
}

pub(crate) fn handle_interrupt_flag(state: &State, controller_state: &mut ControllerState) -> ControllerResult {
    state.cdrom.interrupt_flag.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => Ok(value),
            LatchKind::Write => {
                let acknowledge_interrupt = INTERRUPT_FLAGS.extract_from(value) as usize;
                controller_state.interrupt_index = INTERRUPT_FLAGS.acknowledge(controller_state.interrupt_index, acknowledge_interrupt);

                if acknowledge_interrupt > 0 {
                    if controller_state.interrupt_index > 0 {
                        return Err(format!("Interrupt still pending after acknowledgement: {}", controller_state.interrupt_index));
                    }

                    state.cdrom.response.clear();
                }

                calculate_interrupt_flag_value(controller_state)
            },
        }
    })
}
