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

pub fn handle_transfer(state: &State, dmac_state: &mut ControllerState, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    handle_transfer_start(state, dmac_state, channel_id);
    
    let handler = {
        let transfer_state = get_transfer_state(dmac_state, channel_id);

        if transfer_state.started {
             match transfer_state.sync_mode_state {
                SyncModeState::Undefined => unreachable!(),
                SyncModeState::Continuous(_) => handle_continuous_transfer,
                SyncModeState::Blocks(_) => handle_blocks_transfer,
                SyncModeState::LinkedList(_) => handle_linked_list_transfer,
            }
        } else {
            return Ok(0);
        }
    };

    handler(state, dmac_state, channel_id, word_transfers_allowed)
}

fn handle_transfer_start(state: &State, dmac_state: &mut ControllerState, channel_id: usize) {
    let chcr = get_chcr(state, channel_id);
    let madr = get_madr(state, channel_id);
    let bcr = get_bcr(state, channel_id);
    let transfer_state = get_transfer_state(dmac_state, channel_id);

    if chcr.write_latch.load(Ordering::Acquire) {
        assert!(!transfer_state.started, format!("DMA transfer already started, channel_id = {}", channel_id));

        if chcr.register.read_bitfield(CHCR_STARTBUSY) != 0 {
            if chcr.register.read_bitfield(CHCR_CHOPPING) != 0 {
                unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel_id = {})", channel_id);
            }

            state.bus_locked.store(true, Ordering::Release);

            initialize_transfer_state(transfer_state, chcr, madr, bcr);

            debug::transfer_start(state, dmac_state, channel_id);
        }

        chcr.write_latch.store(false, Ordering::Release)
    }
}

fn handle_transfer_finish(state: &State, dmac_state: &mut ControllerState, channel_id: usize, bcr_value: Option<u32>, madr_value: Option<u32>) {
    let chcr = get_chcr(state, channel_id);
    let madr = get_madr(state, channel_id);
    let bcr = get_bcr(state, channel_id);
    let transfer_state = get_transfer_state(dmac_state, channel_id);

    transfer_state.started = false;

    chcr.register.write_bitfield(CHCR_STARTBUSY, 0);

    if let Some(value) = bcr_value {
        bcr.write_u32(value);
    }

    if let Some(value) = madr_value {
        madr.write_u32(value);
    }

    raise_irq(state, channel_id);

    debug::transfer_end(state, dmac_state, channel_id);
}

fn handle_continuous_transfer(state: &State, dmac_state: &mut ControllerState, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let transfer_state = {
        let transfer_state = get_transfer_state(dmac_state, channel_id);
        if let SyncModeState::Continuous(ref mut s) = transfer_state.sync_mode_state {
            s
        } else {
            panic!();
        }
    };

    let word_transfers_allowed = min(word_transfers_allowed, transfer_state.transfers_remaining());
    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        match transfer_direction {
            TransferDirection::FromChannel => {
                let last_transfer = transfer_state.transfers_remaining() == 1;
                let value = pop_channel_data(state, channel_id, transfer_state.current_address, last_transfer).map_err(|_| word_transfers_count)?;
                state.memory.main_memory.write_u32(transfer_state.current_address, value);
            },
            TransferDirection::ToChannel => {
                let value = state.memory.main_memory.read_u32(transfer_state.current_address);
                push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
            },
        }

        transfer_state.increment(madr_step_direction);
        word_transfers_count += 1;
    }

    if transfer_state.transfers_remaining() == 0 {
        handle_transfer_finish(state, dmac_state, channel_id, None, None);
    }

    Ok(word_transfers_count)
}

fn handle_blocks_transfer(state: &State, dmac_state: &mut ControllerState, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let transfer_state = {
        let transfer_state = get_transfer_state(dmac_state, channel_id);
        if let SyncModeState::Blocks(ref mut s) = transfer_state.sync_mode_state {
            s
        } else {
            panic!();
        }
    };

    let (word_transfers_count, finished) = {
        let word_transfers_allowed = min(word_transfers_allowed, transfer_state.transfers_remaining());
        let mut word_transfers_count = 0;
    
        while word_transfers_count < word_transfers_allowed {
            match transfer_direction {
                TransferDirection::FromChannel => {
                    let last_transfer = transfer_state.transfers_remaining() == 1;
                    let value = pop_channel_data(state, channel_id, transfer_state.current_address, last_transfer).map_err(|_| word_transfers_count)?;
                    state.memory.main_memory.write_u32(transfer_state.current_address, value);
                },
                TransferDirection::ToChannel => {
                    let value = state.memory.main_memory.read_u32(transfer_state.current_address);
                    push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
                },
            }
    
            transfer_state.increment(madr_step_direction);
            word_transfers_count += 1;
        }
    
        let finished = transfer_state.transfers_remaining() == 0;
        (word_transfers_count, finished)
    };

    if finished {
        let madr_value = Some(transfer_state.current_address);
        handle_transfer_finish(state, dmac_state, channel_id, Some(0), madr_value);
    }

    Ok(word_transfers_count)
}

fn handle_linked_list_transfer(state: &State, dmac_state: &mut ControllerState, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(state, channel_id);
    let transfer_direction = get_transfer_direction(chcr);

    assert!(transfer_direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");

    let transfer_state = {
        let transfer_state = get_transfer_state(dmac_state, channel_id);
        if let SyncModeState::LinkedList(ref mut s) = transfer_state.sync_mode_state {
            s
        } else {
            panic!();
        }
    };

    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        if transfer_state.transfers_remaining() == 0 {
            if transfer_state.next_header_address == 0xFF_FFFF {
                handle_transfer_finish(state, dmac_state, channel_id, None, Some(0xFF_FFFF));
                break;
            }

            match linked_list::process_header(transfer_state, &state.memory.main_memory) {
                Err(()) => {
                    warn!("Linked list transfer: null pointer encountered, ending transfer prematurely");
                    handle_transfer_finish(state, dmac_state, channel_id, None, Some(0xFF_FFFF));
                    break;
                },
                Ok(()) => {},
            }

            word_transfers_count += 1;
        } else {
            let address = (transfer_state.current_header_address + DATA_SIZE) + ((transfer_state.current_count as u32) * DATA_SIZE);
            let value = state.memory.main_memory.read_u32(address as u32);
            push_channel_data(state, channel_id, value).map_err(|_| word_transfers_count)?;
            transfer_state.increment();
            word_transfers_count += 1;
        }
    }

    Ok(word_transfers_count)
}
