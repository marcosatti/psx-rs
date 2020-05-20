use crate::system::{
    gpu::types::*,
    types::State,
};

pub(crate) fn handle_read(state: &State, controller_state: &mut ControllerState) {
    let read_buffer = &mut controller_state.gp0_read_buffer;
    let read = &state.gpu.read;

    loop {
        if read.is_full() {
            break;
        }

        match read_buffer.pop_front() {
            Some(v) => read.write_one(v).unwrap(),
            None => break,
        }
    }
}
