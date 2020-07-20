use crate::system::{
    dmac::{
        constants::*,
        controllers::fifo::*,
        types::*,
    },
    types::{ControllerResult, State},
};

pub(crate) fn handle_transfer(
    state: &State, continuous_state: &mut ContinuousState, channel_id: usize, transfer_direction: TransferDirection, step_direction: StepDirection,
) -> ControllerResult<(bool, bool)> {
    match transfer_direction {
        TransferDirection::FromChannel => {
            let last_transfer = transfers_remaining(continuous_state) == 1;
            let value = match pop_channel_data(state, channel_id, continuous_state.current_address, last_transfer)? {
                Some(v) => v,
                None => return Ok((true, false)),
            };
            state.memory.main_memory.write_u32(continuous_state.current_address, value);
        },
        TransferDirection::ToChannel => {
            let value = state.memory.main_memory.read_u32(continuous_state.current_address);
            if let None = push_channel_data(state, channel_id, value)? {
                return Ok((true, false));
            }
        },
    }

    match step_direction {
        StepDirection::Forwards => continuous_state.current_address += DATA_SIZE,
        StepDirection::Backwards => continuous_state.current_address -= DATA_SIZE,
    }

    continuous_state.current_count += 1;

    let finished = transfers_remaining(continuous_state) == 0;

    Ok((false, finished))
}

fn transfers_remaining(continuous_state: &mut ContinuousState) -> usize {
    continuous_state.target_count - continuous_state.current_count
}
