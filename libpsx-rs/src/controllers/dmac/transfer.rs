use std::sync::atomic::Ordering;
use std::cmp::min;
use log::warn;
use crate::resources::Resources;
use crate::controllers::dmac::channel::*;
use crate::constants::dmac::*;
use crate::resources::dmac::*;
use crate::resources::dmac::channel::*;
use crate::controllers::dmac::debug;
use crate::controllers::dmac::linked_list;

pub fn handle_transfer(resources: &mut Resources, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let transfer_state = get_transfer_state(resources, channel_id);

    handle_transfer_start(resources, channel_id);

    if transfer_state.started {
        match transfer_state.sync_mode_state {
            SyncModeState::Undefined => unreachable!(),
            SyncModeState::Continuous(ref mut s) => handle_continuous_transfer(s, resources, channel_id, word_transfers_allowed),
            SyncModeState::Blocks(ref mut s) => handle_blocks_transfer(s, resources, channel_id, word_transfers_allowed),
            SyncModeState::LinkedList(ref mut s) => handle_linked_list_transfer(s, resources, channel_id, word_transfers_allowed),
        }
    } else {
        Ok(0)
    }
}

fn handle_transfer_start(resources: &mut Resources, channel_id: usize) {
    let chcr = get_chcr(resources, channel_id);
    let madr = get_madr(resources, channel_id);
    let bcr = get_bcr(resources, channel_id);
    let transfer_state = get_transfer_state(resources, channel_id);

    if chcr.write_latch.load(Ordering::Acquire) {
        assert!(!transfer_state.started, format!("DMA transfer already started, channel_id = {}", channel_id));

        if chcr.register.read_bitfield(CHCR_STARTBUSY) != 0 {
            if channel_id == 3 { log::debug!("CDROM starting"); }

            if chcr.register.read_bitfield(CHCR_CHOPPING) != 0 {
                unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel_id = {})", channel_id);
            }

            resources.bus_locked.store(true, Ordering::Release);

            initialize_transfer_state(transfer_state, chcr, madr, bcr);

            debug::transfer_start(resources, channel_id);
        }

        chcr.write_latch.store(false, Ordering::Release)
    }
}

fn handle_transfer_finish(resources: &mut Resources, channel_id: usize, bcr_value: Option<u32>, madr_value: Option<u32>) {
    let chcr = get_chcr(resources, channel_id);
    let madr = get_madr(resources, channel_id);
    let bcr = get_bcr(resources, channel_id);
    let transfer_state = get_transfer_state(resources, channel_id);
    
    if channel_id == 3 { log::debug!("CDROM finishing"); }

    transfer_state.started = false;

    chcr.register.write_bitfield(CHCR_STARTBUSY, 0);
    
    if let Some(value) = bcr_value {
        bcr.write_u32(value);
    }

    if let Some(value) = madr_value {
        madr.write_u32(value);
    }

    raise_irq(resources, channel_id);

    debug::transfer_end(resources, channel_id);
}

fn handle_continuous_transfer(state: &mut ContinuousState, resources: &mut Resources, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(resources, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let word_transfers_allowed = min(word_transfers_allowed, state.transfers_remaining());
    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        match transfer_direction {
            TransferDirection::FromChannel => {
                let last_transfer = state.transfers_remaining() == 1;
                let value = { 
                    let result = pop_channel_data(resources, channel_id, state.current_address, last_transfer).map_err(|_| word_transfers_count);
                    if result.is_err() {
                        log::debug!("error, transfers remaining = {}, this count = {}", state.transfers_remaining(), word_transfers_count);
                        return Err(result.unwrap_err());
                    }
                    result.unwrap()
                };
                resources.main_memory.write_u32(state.current_address, value);
            },
            TransferDirection::ToChannel => {
                let value = resources.main_memory.read_u32(state.current_address);
                push_channel_data(resources, channel_id, value).map_err(|_| word_transfers_count)?;
            },
        }
    
        state.increment(madr_step_direction);
        word_transfers_count += 1;
    }

    log::debug!("transfers remaining: {}", state.transfers_remaining());

    if state.transfers_remaining() == 0 {
        handle_transfer_finish(resources, channel_id, None, None);
    }

    Ok(word_transfers_count)
}

fn handle_blocks_transfer(state: &mut BlocksState, resources: &mut Resources, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(resources, channel_id);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let word_transfers_allowed = min(word_transfers_allowed, state.transfers_remaining());
    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        match transfer_direction {
            TransferDirection::FromChannel => {
                let last_transfer = state.transfers_remaining() == 1;
                let value = pop_channel_data(resources, channel_id, state.current_address, last_transfer).map_err(|_| word_transfers_count)?;
                resources.main_memory.write_u32(state.current_address, value);
            },
            TransferDirection::ToChannel => {
                let value = resources.main_memory.read_u32(state.current_address);
                push_channel_data(resources, channel_id, value).map_err(|_| word_transfers_count)?;
            },
        }

        state.increment(madr_step_direction);
        word_transfers_count += 1;
    }

    if state.transfers_remaining() == 0 {
        handle_transfer_finish(resources, channel_id, Some(0), Some(state.current_address));
    }

    Ok(word_transfers_count)
}

fn handle_linked_list_transfer(state: &mut LinkedListState, resources: &mut Resources, channel_id: usize, word_transfers_allowed: usize) -> Result<usize, usize> {
    let chcr = get_chcr(resources, channel_id);
    let transfer_direction = get_transfer_direction(chcr);

    assert!(transfer_direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");

    let mut word_transfers_count = 0;

    while word_transfers_count < word_transfers_allowed {
        if state.transfers_remaining() == 0 {
            if state.next_header_address == 0xFF_FFFF {
                handle_transfer_finish(resources, channel_id, None, Some(0xFF_FFFF));
                break;
            }

            match linked_list::process_header(state, &resources.main_memory) {
                Err(()) => {
                    warn!("Linked list transfer: null pointer encountered, ending transfer prematurely");
                    handle_transfer_finish(resources, channel_id, None, Some(0xFF_FFFF));
                    break;
                },
                Ok(()) => {},
            }

            word_transfers_count += 1;
        } else {
            let address = (state.current_header_address + DATA_SIZE) + ((state.current_count as u32) * DATA_SIZE);
            let value = resources.main_memory.read_u32(address as u32);
            push_channel_data(resources, channel_id, value).map_err(|_| word_transfers_count)?;
            state.increment();
            word_transfers_count += 1;
        }
    }

    Ok(word_transfers_count)
}
