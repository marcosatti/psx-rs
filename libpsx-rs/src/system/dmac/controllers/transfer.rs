use crate::system::{
    dmac::{
        constants::*,
        controllers::{
            channel::*,
            debug,
            linked_list,
        },
        types::*,
    },
    types::State,
};
use log::warn;
use std::{
    cmp::min,
    sync::atomic::Ordering,
};

pub fn handle_transfer(state: &mut State, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let transfer_state = get_transfer_state(state, channel_id);

    handle_transfer_start(state, channel_id);

    if transfer_state.started {
        match transfer_state.sync_mode_state {
            SyncModeState::Undefined => unreachable!(),
            SyncModeState::Continuous(ref mut s) => handle_continuous_transfer(s, state, channel_id, word_transfers_allowed),
            SyncModeState::Blocks(ref mut s) => handle_blocks_transfer(s, state, channel_id, word_transfers_allowed),
            SyncModeState::LinkedList(ref mut s) => handle_linked_list_transfer(s, state, channel_id, word_transfers_allowed),
        }
    } else {
        Ok(0)
    }
}

fn handle_transfer_start(state: &mut State, channel_id: usize) {
    let chcr = get_chcr(state, channel_id);
    let madr = get_madr(state, channel_id);
    let bcr = get_bcr(state, channel_id);
    let transfer_state = get_transfer_state(state, channel_id);

    if chcr.write_latch.load(Ordering::Acquire) {
        assert!(!transfer_state.started, format!("DMA transfer already started, channel_id = {}", channel_id));

        if chcr.register.read_bitfield(CHCR_STARTBUSY) != 0 {
            if chcr.register.read_bitfield(CHCR_CHOPPING) != 0 {
                unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel_id = {})", channel_id);
            }

            state.bus_locked.store(true, Ordering::Release);

            initialize_transfer_state(transfer_state, chcr, madr, bcr);

            debug::transfer_start(state, channel_id);
        }

        chcr.write_latch.store(false, Ordering::Release)
    }
}

fn handle_transfer_finish(state: &mut State, channel_id: usize, bcr_value: Option<u32>, madr_value: Option<u32>) {
    let chcr = get_chcr(state, channel_id);
    let madr = get_madr(state, channel_id);
    let bcr = get_bcr(state, channel_id);
    let transfer_state = get_transfer_state(state, channel_id);

    transfer_state.started = false;

    chcr.register.write_bitfield(CHCR_STARTBUSY, 0);

    if let Some(value) = bcr_value {
        bcr.write_u32(value);
    }

    if let Some(value) = madr_value {
        madr.write_u32(value);
    }

    raise_irq(state, channel_id);

    debug::transfer_end(state, channel_id);
}

fn handle_continuous_transfer(transfer_state: &mut ContinuousState, state: &mut State, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let word_transfers_allowed = min(word_transfers_allowed, transfer_state.transfers_remaining());
    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        match transfer_direction {
            TransferDirection::FromChannel => {
                let last_transfer = transfer_state.transfers_remaining() == 1;
                let value = pop_channel_data(state, channel_id, transfer_state.current_address, last_transfer).map_err(|_| word_transfers_count)?;
                state.main_memory.write_u32(transfer_state.current_address, value);
            },
            TransferDirection::ToChannel => {
                let value = state.main_memory.read_u32(transfer_state.current_address);
                push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
            },
        }

        transfer_state.increment(madr_step_direction);
        word_transfers_count += 1;
    }

    if transfer_state.transfers_remaining() == 0 {
        handle_transfer_finish(state, channel_id, None, None);
    }

    Ok(word_transfers_count)
}

fn handle_blocks_transfer(transfer_state: &mut BlocksState, state: &mut State, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let word_transfers_allowed = min(word_transfers_allowed, transfer_state.transfers_remaining());
    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        match transfer_direction {
            TransferDirection::FromChannel => {
                let last_transfer = transfer_state.transfers_remaining() == 1;
                let value = pop_channel_data(state, channel_id, transfer_state.current_address, last_transfer).map_err(|_| word_transfers_count)?;
                state.main_memory.write_u32(transfer_state.current_address, value);
            },
            TransferDirection::ToChannel => {
                let value = state.main_memory.read_u32(transfer_state.current_address);
                push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
            },
        }

        transfer_state.increment(madr_step_direction);
        word_transfers_count += 1;
    }

    if transfer_state.transfers_remaining() == 0 {
        handle_transfer_finish(state, channel_id, Some(0), Some(transfer_state.current_address));
    }

    Ok(word_transfers_count)
}

fn handle_linked_list_transfer(transfer_state: &mut LinkedListState, state: &mut State, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);

    assert!(transfer_direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");

    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        if transfer_state.transfers_remaining() == 0 {
            if transfer_state.next_header_address == 0xFF_FFFF {
                handle_transfer_finish(state, channel_id, None, Some(0xFF_FFFF));
                break;
            }

            match linked_list::process_header(transfer_state, &state.main_memory) {
                Err(()) => {
                    warn!("Linked list transfer: null pointer encountered, ending transfer prematurely");
                    handle_transfer_finish(state, channel_id, None, Some(0xFF_FFFF));
                    break;
                },
                Ok(()) => {},
            }

            word_transfers_count += 1;
        } else {
            let address = (transfer_state.current_header_address + DATA_SIZE) + ((transfer_state.current_count as u32) * DATA_SIZE);
            let value = state.main_memory.read_u32(address as u32);
            push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
            transfer_state.increment();
            word_transfers_count += 1;
        }
    }

    Ok(word_transfers_count)
}
