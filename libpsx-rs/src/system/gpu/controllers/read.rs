use crate::system::{
    gpu::types::*,
    types::{
        ControllerResult,
        State,
    },
};

pub(crate) fn handle_read(state: &State, controller_state: &mut ControllerState) -> ControllerResult<bool> {
    let read_buffer = &mut controller_state.gp0_read_buffer;
    let read = &state.gpu.read;
    let mut handled = false;

    loop {
        if read.is_full() {
            break;
        }

        match read_buffer.pop_front() {
            Some(v) => {
                read.write_one(v).map_err(|_| "Error writing to GPUREAD FIFO".to_owned())?;
                handled = true;
            },
            None => break,
        }
    }

    Ok(handled)
}
