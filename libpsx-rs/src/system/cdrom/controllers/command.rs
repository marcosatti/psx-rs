use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            controllers::command_impl,
            types::*,
        },
        types::{
            ControllerResult,
            State,
        },
    },
};

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) -> ControllerResult<()> {
    if controller_state.command_index.is_none() {
        return Ok(());
    }

    let command_index = controller_state.command_index.unwrap();
    let command_iteration = controller_state.command_iteration;
    let handler = get_handler_fn(command_index)?;

    match controller_state.command_wait_cycles {
        WaitCyclesMode::Ready => {
            let cycles = (handler.0)(command_iteration)?;
            controller_state.command_wait_cycles = WaitCyclesMode::Waiting(cycles);
            log::debug!("Waiting 0x{:X} cycles", cycles);
            return Ok(());
        },
        WaitCyclesMode::Waiting(ref mut cycles) => {
            *cycles -= 1;
            if *cycles == 0 {
                controller_state.command_wait_cycles = WaitCyclesMode::Executing;
            }
            return Ok(());
        },
        WaitCyclesMode::Executing => {},
    }

    let parameter_count = state.cdrom.parameter.read_available();
    let parameters_needed = (handler.1)(command_iteration);
    if parameter_count < parameters_needed {
        return Err(format!("Parameter FIFO did not have enough data for command {:X}: need {}, have {}", command_index, parameters_needed, parameter_count));
    }

    if !state.cdrom.response.is_empty() {
        return Err("CDROM response FIFO still had bytes when a new command was run!".into());
    }

    log::debug!("Executing command {:02X} iteration {}", command_index, command_iteration);
    let finished = (handler.2)(state, controller_state, cdrom_backend, command_iteration)?;

    if finished {
        controller_state.command_index = None;
        controller_state.command_iteration = 0;
    } else {
        controller_state.command_iteration += 1;
    }

    controller_state.command_wait_cycles = WaitCyclesMode::Ready;

    if !state.cdrom.parameter.is_empty() {
        return Err("CDROM parameter FIFO still had bytes when a command was just run!".into());
    }

    Ok(())
}

fn get_handler_fn(command_index: u8) -> ControllerResult<(WaitCyclesFn, LengthFn, HandlerFn)> {
    match command_index {
        0x01 => Ok((command_impl::default_wait_cycles, command_impl::command_01_length, command_impl::command_01_handler)),
        0x02 => Ok((command_impl::default_wait_cycles, command_impl::command_02_length, command_impl::command_02_handler)),
        0x06 => Ok((command_impl::default_wait_cycles, command_impl::command_06_length, command_impl::command_06_handler)),
        0x08 => Ok((command_impl::default_wait_cycles, command_impl::command_08_length, command_impl::command_08_handler)),
        0x09 => Ok((command_impl::default_wait_cycles, command_impl::command_09_length, command_impl::command_09_handler)),
        0x0A => Ok((command_impl::command_0a_wait_cycles, command_impl::command_0a_length, command_impl::command_0a_handler)),
        0x0E => Ok((command_impl::default_wait_cycles, command_impl::command_0e_length, command_impl::command_0e_handler)),
        0x15 => Ok((command_impl::default_wait_cycles, command_impl::command_15_length, command_impl::command_15_handler)),
        0x19 => Ok((command_impl::default_wait_cycles, command_impl::command_19_length, command_impl::command_19_handler)),
        0x1A => Ok((command_impl::default_wait_cycles, command_impl::command_1a_length, command_impl::command_1a_handler)),
        _ => Err(format!("Command not implemented: 0x{:0X}", command_index)),
    }
}
