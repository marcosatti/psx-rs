use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::command_impl,
        },
        types::State,
    },
};
use std::sync::atomic::Ordering;

type LengthFn = fn(usize) -> usize;

type HandlerFn = fn(&mut State, &CdromBackend, usize) -> bool;

pub fn handle_command(state: &mut State, cdrom_backend: &CdromBackend) -> bool {
    if state.cdrom.command_index.is_none() {
        // Read a new command if available.
        if !state.cdrom.command.write_latch.load(Ordering::Acquire) {
            return false;
        }

        state.cdrom.status.write_bitfield(STATUS_BUSYSTS, 1);
        let command_value = state.cdrom.command.register.read_u8();

        state.cdrom.command.write_latch.store(false, Ordering::Release);

        state.cdrom.command_index = Some(command_value);
        state.cdrom.command_iteration = 0;
    }

    let command_index = state.cdrom.command_index.unwrap();
    let command_iteration = state.cdrom.command_iteration;
    let handler = get_handler_fn(command_index);

    let parameter_count = state.cdrom.parameter.read_available();
    if parameter_count < (handler.0)(command_iteration) {
        return false;
    }

    assert!(
        state.cdrom.response.read_available() == 0,
        "CDROM response FIFO still had bytes when a new command was run!"
    );

    let finished = (handler.1)(state, cdrom_backend, command_iteration);

    if !finished {
        state.cdrom.command_iteration += 1;
    } else {
        state.cdrom.command_index = None;
    }

    assert!(
        state.cdrom.parameter.read_available() == 0,
        "CDROM parameter FIFO still had bytes when a command was just run!"
    );

    state.cdrom.status.write_bitfield(STATUS_BUSYSTS, 0);

    true
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
