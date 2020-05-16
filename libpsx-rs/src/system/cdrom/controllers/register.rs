use crate::{
    system::{
        cdrom::{
            constants::*,
            controllers::interrupt::*,
            types::*,
        },
        types::State,
    },
    types::memory::*,
};

pub fn handle_command(state: &State, controller_state: &mut ControllerState) {
    state.cdrom.command.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::None => unreachable!(),
            LatchKind::Read => value,
            LatchKind::Write => {
                assert!(controller_state.command_index.is_none());
                controller_state.command_index = Some(value);
                value
            },
        }
    });
}

pub fn handle_request(state: &State, controller_state: &mut ControllerState) {
    state.cdrom.request.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::None => unreachable!(),
            LatchKind::Read => value,
            LatchKind::Write => {
                if REQUEST_SMEN.extract_from(value) > 0 {
                    unimplemented!("Command start interrupt");
                }

                if REQUEST_BFRD.extract_from(value) > 0 {
                    controller_state.load_data_flag = true;
                // log::debug!("Load data FIFO set");
                } else {
                    assert_eq!(state.cdrom.data.read_available(), 0);
                    state.cdrom.data.clear();
                    // log::debug!("Reset data FIFO");
                }

                0
            },
        }
    });
}

pub fn handle_interrupt_flag(state: &State, controller_state: &mut ControllerState) {
    state.cdrom.interrupt_flag.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::None => unreachable!(),
            LatchKind::Read => value,
            LatchKind::Write => {
                let acknowledge_interrupt = INTERRUPT_FLAGS.extract_from(value) as usize;
                controller_state.interrupt_index = INTERRUPT_FLAGS.acknowledge(controller_state.interrupt_index, acknowledge_interrupt);

                if acknowledge_interrupt > 0 {
                    assert_eq!(controller_state.interrupt_index, 0);
                    state.cdrom.response.clear();
                    // log::debug!("Cleared response FIFO");
                }

                calculate_interrupt_flag_value(controller_state)
            },
        }
    });
}
