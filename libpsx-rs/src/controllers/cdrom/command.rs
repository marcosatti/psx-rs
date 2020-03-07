use std::sync::atomic::Ordering;
use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::resources::cdrom::*;
use crate::controllers::cdrom::command_impl;

type LengthFn = fn(usize) -> usize;

type HandlerFn = fn(&mut Resources, &CdromBackend, usize) -> bool;

pub fn handle_command(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) -> bool {
    if resources.cdrom.command_index.is_none() {
        // Read a new command if available.
        if !resources.cdrom.command.write_latch.load(Ordering::Acquire) {
            return false;
        }

        resources.cdrom.status.write_bitfield(STATUS_BUSYSTS, 1);
        let command_value = resources.cdrom.command.register.read_u8();
        
        resources.cdrom.command.write_latch.store(false, Ordering::Release);

        resources.cdrom.command_index = Some(command_value);
        resources.cdrom.command_iteration = 0;
    }

    let command_index = resources.cdrom.command_index.unwrap();
    let command_iteration = resources.cdrom.command_iteration;
    let handler = get_handler_fn(command_index);

    let parameter_count = resources.cdrom.parameter.read_available();
    if parameter_count < (handler.0)(command_iteration) {
        return false;
    }

    assert!(resources.cdrom.response.read_available() == 0, "CDROM response FIFO still had bytes when a new command was run!");

    log::debug!("Running command {:X}, iter = {}", command_index, command_iteration);
    let finished = (handler.1)(resources, cdrom_backend, command_iteration);

    if !finished {
        resources.cdrom.command_iteration += 1;
    } else {
        resources.cdrom.command_index = None;
    }

    assert!(resources.cdrom.parameter.read_available() == 0, "CDROM parameter FIFO still had bytes when a command was just run!");

    resources.cdrom.status.write_bitfield(STATUS_BUSYSTS, 0);

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
