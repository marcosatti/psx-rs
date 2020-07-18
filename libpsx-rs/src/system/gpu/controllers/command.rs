use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            controllers::{
                command_gp0::handle_command as handle_command_gp0,
                command_gp1::handle_command as handle_command_gp1,
            },
            types::ControllerState,
        },
        types::{ControllerResult, State},
    },
};

pub(crate) fn handle_command(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend) -> ControllerResult {
    // TODO: what's the priority of command handling?
    // Doesn't really mention what happens if there is a command waiting in GP0 queue then a command gets written to
    // GP1.
    handle_command_gp1(state, controller_state, video_backend)?;
    handle_command_gp0(state, controller_state, video_backend)?;
    
    Ok(())
}
