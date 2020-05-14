use crate::system::types::State;
use crate::system::cdrom::types::*;
use crate::system::cdrom::constants::*;
use crate::system::cdrom::controllers::interrupt::*;
use crate::types::memory::*;

pub fn handle_command(state: &State, controller_state: &mut ControllerState) {
    state.cdrom.command.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => {
                assert!(controller_state.command_index.is_none());
                controller_state.command_index = Some(value);
                value
            },
        }
    });
}

pub fn handle_request(state: &State, _controller_state: &mut ControllerState) {
    state.cdrom.request.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => {
                unimplemented!();
            },
        }
    });
}

pub fn handle_interrupt_flag(state: &State, controller_state: &mut ControllerState) {
    state.cdrom.interrupt_flag.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => {
                if INT_FLAG_CLRPRM.extract_from(value) > 0 {
                    unimplemented!();
                }

                let acknowledge_interrupt = INTERRUPT_FLAGS.extract_from(value) as usize;
                if acknowledge_interrupt != controller_state.interrupt_index {
                    panic!("Raised interrupt {} but acknowledgement was {}", controller_state.interrupt_index, acknowledge_interrupt);
                }

                calculate_interrupt_flag_value(controller_state)
            },
        }
    });
}
