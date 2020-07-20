use crate::{
    system::{
        gpu::types::*,
        types::{
            ControllerResult,
            State,
        },
    },
    types::memory::*,
};

pub(crate) fn handle_gp1(state: &State, controller_state: &mut ControllerState) -> ControllerResult<()> {
    state.gpu.gp1.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => unreachable!(),
            LatchKind::Write => {
                if controller_state.gp1_command.is_some() {
                    return Err("GP1 command still pending".into());
                }

                controller_state.gp1_command = Some(value);
                Ok(value)
            },
        }
    })
}
