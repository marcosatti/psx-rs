use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            controllers::command_gp1_impl,
            types::ControllerState,
        },
        types::{
            ControllerResult,
            State,
        },
    },
};

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend) -> ControllerResult<bool> {
    if !state.gpu.gp1_command_pending.load() {
        return Ok(false);
    }

    let command_value = state.gpu.gp1.read_u32();
    let command_index = GP_CMD.extract_from(command_value) as u8;

    let command_fn = match command_index {
        0x00 => command_gp1_impl::command_00,
        0x01 => command_gp1_impl::command_01,
        0x02 => command_gp1_impl::command_02,
        0x03 => command_gp1_impl::command_03,
        0x04 => command_gp1_impl::command_04,
        0x05 => command_gp1_impl::command_05,
        0x06 => command_gp1_impl::command_06,
        0x07 => command_gp1_impl::command_07,
        0x08 => command_gp1_impl::command_08,
        0x10 => command_gp1_impl::command_10,
        _ => return Err(format!("Unknown GP1 command: 0x{:0X}", command_index)),
    };

    command_fn(state, controller_state, video_backend, command_value)?;

    state.gpu.gp1_command_pending.store(false);

    Ok(true)
}
