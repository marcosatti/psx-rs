use std::sync::atomic::Ordering;
use crate::resources::Resources;
use crate::controllers::dmac::channel::*;
use crate::constants::dmac::*;
use crate::resources::dmac::*;
use crate::resources::dmac::channel::*;
use crate::controllers::dmac::debug;
use crate::controllers::dmac::linked_list;

pub fn handle_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let transfer_state = get_transfer_state(resources, channel);
    let chcr = get_chcr(resources, channel);
    let madr = get_madr(resources, channel);
    let bcr = get_bcr(resources, channel);

    if chcr.read_bitfield(CHCR_CHOPPING) != 0 {
        unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel = {})", channel);
    }

    if chcr.read_bitfield(CHCR_STARTBUSY) != 0 {
        if !transfer_state.started {
            resources.bus_locked.store(true, Ordering::Release);

            initialize_transfer(transfer_state, chcr, madr, bcr);

            debug::transfer_start(resources, channel);
        }

        match transfer_state.sync_mode_state {
            SyncModeState::Undefined => unreachable!(),
            SyncModeState::Continuous(ref mut s) => handle_continuous_transfer(resources, channel),
            SyncModeState::Blocks(ref mut s) => handle_blocks_transfer(resources, channel),
            SyncModeState::LinkedList(ref mut s) => handle_linked_list_transfer(resources, channel),
        }
    } else {
        0
    }
}

fn handle_continuous_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = get_chcr(resources, channel);
    let transfer_state = get_transfer_state(resources, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let continuous_state = if let SyncModeState::Continuous(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = (continuous_state.target_count - continuous_state.current_count) == 1;

    match transfer_direction {
        TransferDirection::FromChannel => {
            let result = pop_channel_data(resources, channel, continuous_state.current_address, last_transfer);
            if result.is_err() {
                return 0;
            }
            let value = result.unwrap();
            resources.main_memory.write_u32(continuous_state.current_address as u32, value);
        },
        TransferDirection::ToChannel => {
            let value = resources.main_memory.read_u32(continuous_state.current_address as u32);
            let result = push_channel_data(resources, channel, value);
            if result.is_err() {
                return 0;
            }
        },
    }

    match madr_step_direction {
        StepDirection::Forwards => continuous_state.current_address += DATA_SIZE,
        StepDirection::Backwards => continuous_state.current_address -= DATA_SIZE,
    }

    continuous_state.current_count += 1;

    let finished = (continuous_state.target_count - continuous_state.current_count) == 0;

    if finished {
        transfer_state.started = false;
        chcr.write_bitfield(CHCR_STARTBUSY, 0);
        raise_irq(resources, channel);

        debug::transfer_end(resources, channel);
    }

    return 1;
}

fn handle_blocks_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = get_chcr(resources, channel);
    let transfer_state = get_transfer_state(resources, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);
    let bcr = get_bcr(resources, channel);
    let madr = get_madr(resources, channel);

    let blocks_state = if let SyncModeState::Blocks(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = {
        let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
        let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
        (target - current) == 1
    };

    match transfer_direction {
        TransferDirection::FromChannel => {
            let result = pop_channel_data(resources, channel, blocks_state.current_address, last_transfer);
            if result.is_err() {
                return 0;
            }
            let value = result.unwrap();
            resources.main_memory.write_u32(blocks_state.current_address as u32, value);
        },
        TransferDirection::ToChannel => {
            let value = resources.main_memory.read_u32(blocks_state.current_address as u32);
            let result = push_channel_data(resources, channel, value);
            if result.is_err() {
                return 0;
            }
        },
    }

    match madr_step_direction {
        StepDirection::Forwards => blocks_state.current_address += DATA_SIZE,
        StepDirection::Backwards => blocks_state.current_address -= DATA_SIZE,
    }

    blocks_state.current_bsize_count += 1;
    if blocks_state.current_bsize_count == blocks_state.target_bsize_count {
        blocks_state.current_bsize_count = 0;
        blocks_state.current_bamount_count += 1;
    }

    let finished = {
        let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
        let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
        (target - current) == 0
    };

    if finished {
        transfer_state.started = false;
        madr.write_u32(blocks_state.current_address as u32);
        bcr.write_bitfield(BCR_BLOCKAMOUNT, 0);
        chcr.write_bitfield(CHCR_STARTBUSY, 0);
        raise_irq(resources, channel);

        debug::transfer_end(resources, channel);
    }

    return 1;
}

fn handle_linked_list_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = get_chcr(resources, channel);
    let transfer_state = get_transfer_state(resources, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr = get_madr(resources, channel);

    assert!(transfer_direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");

    let linked_list_state = if let SyncModeState::LinkedList(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    if linked_list_state.current_count == linked_list_state.target_count {    
        assert!(linked_list_state.next_address != 0x0, "Null linked list transfer address - DMAC is probably too fast for CPU");
        
        if linked_list_state.next_address == 0xFF_FFFF {
            transfer_state.started = false;
            chcr.write_bitfield(CHCR_STARTBUSY, 0);
            madr.write_u32(0xFF_FFFF);
            raise_irq(resources, channel);
            
            debug::transfer_end(resources, channel);

            return 1;
        }

        linked_list::process_header(linked_list_state, &resources.main_memory);
        return 1;
    } else {
        let address = (linked_list_state.current_address + DATA_SIZE) + ((linked_list_state.current_count as u32) * DATA_SIZE);
        let value = resources.main_memory.read_u32(address as u32);
        let result = push_channel_data(resources, channel, value);
        if result.is_err() {
            return 0;
        }
        linked_list_state.current_count += 1;

        return 1;
    }
}
