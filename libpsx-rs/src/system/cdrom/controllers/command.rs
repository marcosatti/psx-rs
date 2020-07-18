use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            controllers::command_impl,
            types::ControllerState,
        },
        types::State,
    },
};

type LengthFn = fn(usize) -> usize;

type HandlerFn = fn(&State, &mut ControllerState, &CdromBackend, usize) -> bool;

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) {
    if controller_state.command_index.is_none() {
        return;
    }

    let command_index = controller_state.command_index.unwrap();
    let command_iteration = controller_state.command_iteration;
    let handler = get_handler_fn(command_index);

    let parameter_count = state.cdrom.parameter.read_available();
    if parameter_count < (handler.0)(command_iteration) {
        panic!("Something is probably wrong in the emulator");
    }

    assert!(state.cdrom.response.is_empty(), "CDROM response FIFO still had bytes when a new command was run!");

    // log::debug!("Executing command {:X}, iteration {}", command_index, command_iteration);
    let finished = (handler.1)(state, controller_state, cdrom_backend, command_iteration);

    if finished {
        controller_state.command_index = None;
        controller_state.command_iteration = 0;
    } else {
        controller_state.command_iteration += 1;
    }

    assert!(state.cdrom.parameter.is_empty(), "CDROM parameter FIFO still had bytes when a command was just run!");
}

fn get_handler_fn(command_index: u8) -> (LengthFn, HandlerFn) {
    match command_index {
        0x01 => (command_impl::command_01_length, command_impl::command_01_handler),
        0x02 => (command_impl::command_02_length, command_impl::command_02_handler),
        0x06 => (command_impl::command_06_length, command_impl::command_06_handler),
        0x09 => (command_impl::command_09_length, command_impl::command_09_handler),
        0x0E => (command_impl::command_0e_length, command_impl::command_0e_handler),
        0x15 => (command_impl::command_15_length, command_impl::command_15_handler),
        0x19 => (command_impl::command_19_length, command_impl::command_19_handler),
        0x1A => (command_impl::command_1a_length, command_impl::command_1a_handler),
        _ => unimplemented!("Command not implemented: 0x{:0X}", command_index),
    }
}
