use crate::{
    system::types::State,
    system::dmac::{
        controllers::fifo::*,
        types::*,
        constants::*,
    },
};

pub fn handle_transfer(state: &State, blocks_state: &mut BlocksState, channel_id: usize, transfer_direction: TransferDirection, step_direction: StepDirection) -> TransferResult {
    match transfer_direction {
        TransferDirection::FromChannel => {
            let last_transfer = transfers_remaining(blocks_state) == 1;
            let value = pop_channel_data(state, channel_id, blocks_state.current_address, last_transfer)?;
            state.memory.main_memory.write_u32(blocks_state.current_address, value);
        },
        TransferDirection::ToChannel => {
            let value = state.memory.main_memory.read_u32(blocks_state.current_address);
            push_channel_data(state, channel_id, value)?;
        },
    }

    match step_direction {
        StepDirection::Forwards => blocks_state.current_address += DATA_SIZE,
        StepDirection::Backwards => blocks_state.current_address -= DATA_SIZE,
    }

    increment(blocks_state);

    let finished = transfers_remaining(blocks_state) == 0;
    Ok(finished)
}

fn increment(blocks_state: &mut BlocksState) {
    blocks_state.current_bsize_count += 1;
    if blocks_state.current_bsize_count == blocks_state.target_bsize_count {
        blocks_state.current_bsize_count = 0;
        blocks_state.current_bamount_count += 1;
    }
}

fn transfers_remaining(blocks_state: &mut BlocksState) -> usize {
    let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
    let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
    target - current
}