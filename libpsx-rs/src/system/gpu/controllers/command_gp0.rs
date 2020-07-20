use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            controllers::command_gp0_impl,
            types::ControllerState,
        },
        types::{
            ControllerResult,
            State,
        },
    },
};

/// Determines the amount of words needed to process the command.
type LengthFn = fn(&[u32]) -> Option<usize>;

/// The handler logic for the command.
type HandlerFn = fn(&State, &mut ControllerState, &VideoBackend, &[u32]) -> ControllerResult<()>;

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend) -> ControllerResult<()> {
    // Update the command buffer with any new incoming data.
    process_gp0_fifo(state, controller_state);

    // Get the associated command handler.
    let command_handler = {
        let command_buffer = &mut controller_state.gp0_command_buffer;

        if command_buffer.is_empty() {
            return Ok(());
        }

        let command = command_buffer[0];
        let command_index = GP_CMD.extract_from(command) as u8;
        get_command_handler(command_index)?
    };

    // Try and get the required data length.
    let required_length_value = {
        let command_buffer = &mut controller_state.gp0_command_buffer;
        let required_length = &mut controller_state.gp0_command_required_length;

        if required_length.is_none() {
            match (command_handler.0)(&command_buffer) {
                Some(command_length) => *required_length = Some(command_length),
                // We don't have enough data yet so try again later.
                None => return Ok(()),
            }
        }

        required_length.unwrap()
    };

    // Check if we can execute the command.
    if controller_state.gp0_command_buffer.len() < required_length_value {
        return Ok(());
    }

    // Execute it.
    let command_buffer_slice = Box::<[u32]>::from(&controller_state.gp0_command_buffer[0..required_length_value]);
    (command_handler.1)(state, controller_state, video_backend, &command_buffer_slice)?;

    // Setup for the next one.
    controller_state.gp0_command_buffer.drain(0..required_length_value);
    controller_state.gp0_command_required_length = None;

    Ok(())
}

fn get_command_handler(command_index: u8) -> ControllerResult<(LengthFn, HandlerFn)> {
    match command_index {
        0x00 => Ok((command_gp0_impl::command_00_length, command_gp0_impl::command_00_handler)),
        0x01 => Ok((command_gp0_impl::command_01_length, command_gp0_impl::command_01_handler)),
        0x02 => Ok((command_gp0_impl::command_02_length, command_gp0_impl::command_02_handler)),
        0x05 => Ok((command_gp0_impl::command_05_length, command_gp0_impl::command_05_handler)),
        0x06 => Ok((command_gp0_impl::command_06_length, command_gp0_impl::command_06_handler)),
        0x0c => Ok((command_gp0_impl::command_0c_length, command_gp0_impl::command_0c_handler)),
        0x28 => Ok((command_gp0_impl::command_28_length, command_gp0_impl::command_28_handler)),
        0x2C => Ok((command_gp0_impl::command_2c_length, command_gp0_impl::command_2c_handler)),
        0x2D => Ok((command_gp0_impl::command_2d_length, command_gp0_impl::command_2d_handler)),
        0x30 => Ok((command_gp0_impl::command_30_length, command_gp0_impl::command_30_handler)),
        0x38 => Ok((command_gp0_impl::command_38_length, command_gp0_impl::command_38_handler)),
        0x3C => Ok((command_gp0_impl::command_3c_length, command_gp0_impl::command_3c_handler)),
        0x50 => Ok((command_gp0_impl::command_50_length, command_gp0_impl::command_50_handler)),
        0x65 => Ok((command_gp0_impl::command_65_length, command_gp0_impl::command_65_handler)),
        0x6F => Ok((command_gp0_impl::command_6f_length, command_gp0_impl::command_6f_handler)),
        0x80 => Ok((command_gp0_impl::command_80_length, command_gp0_impl::command_80_handler)),
        0xA0 => Ok((command_gp0_impl::command_a0_length, command_gp0_impl::command_a0_handler)),
        0xC0 => Ok((command_gp0_impl::command_c0_length, command_gp0_impl::command_c0_handler)),
        0xE1 => Ok((command_gp0_impl::command_e1_length, command_gp0_impl::command_e1_handler)),
        0xE2 => Ok((command_gp0_impl::command_e2_length, command_gp0_impl::command_e2_handler)),
        0xE3 => Ok((command_gp0_impl::command_e3_length, command_gp0_impl::command_e3_handler)),
        0xE4 => Ok((command_gp0_impl::command_e4_length, command_gp0_impl::command_e4_handler)),
        0xE5 => Ok((command_gp0_impl::command_e5_length, command_gp0_impl::command_e5_handler)),
        0xE6 => Ok((command_gp0_impl::command_e6_length, command_gp0_impl::command_e6_handler)),
        _ => Err(format!("Unknown GP0 command: 0x{:0X}", command_index)),
    }
}

fn process_gp0_fifo(state: &State, controller_state: &mut ControllerState) {
    let fifo = &state.gpu.gp0;
    let command_buffer = &mut controller_state.gp0_command_buffer;

    if !fifo.is_empty() {
        while let Ok(v) = fifo.read_one() {
            command_buffer.push(v);
        }
    }
}
