pub(crate) mod blocks;
pub(crate) mod continuous;
pub(crate) mod fifo;
pub(crate) mod linked_list;

use crate::{
    system::{
        dmac::{
            constants::*,
            controllers::{
                channel::*,
                interrupt::*,
            },
            types::*,
        },
        types::State,
    },
    types::bitfield::Bitfield,
};
use std::sync::atomic::Ordering;

pub(crate) fn handle_transfer_initialization(state: &State, transfer_state: &mut TransferState, channel_id: usize) {
    const ADDRESS: Bitfield = Bitfield::new(0, 24);

    let bcr_calculate = |v| {
        match v {
            0 => 0x1_0000,
            _ => v,
        }
    };

    let address = ADDRESS.extract_from(get_madr(state, channel_id).read_u32());
    let bcr_value = get_bcr(state, channel_id).read_u32();
    let bs_count = bcr_calculate(BCR_BLOCKSIZE.extract_from(bcr_value) as usize);
    let ba_count = bcr_calculate(BCR_BLOCKAMOUNT.extract_from(bcr_value) as usize);

    match transfer_state.sync_mode {
        SyncMode::Continuous(ref mut cs) => {
            cs.current_address = address;
            cs.current_count = 0;
            cs.target_count = bs_count;
        },
        SyncMode::Blocks(ref mut bs) => {
            //log::warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending/receiving next block");
            bs.current_address = address;
            bs.current_bsize_count = 0;
            bs.target_bsize_count = bs_count;
            bs.current_bamount_count = 0;
            bs.target_bamount_count = ba_count;
        },
        SyncMode::LinkedList(ref mut lls) => {
            lls.current_header_address = 0;
            lls.next_header_address = address;
            lls.current_count = 0;
            lls.target_count = 0;
        },
        _ => panic!("Undefined sync mode"),
    }
}

pub(crate) fn handle_transfer_finalization(state: &State, transfer_state: &mut TransferState, channel_id: usize) {
    get_chcr(state, channel_id).update(|value| CHCR_STARTBUSY.insert_into(value, 0));

    let madr = get_madr(state, channel_id);
    let bcr = get_bcr(state, channel_id);

    match transfer_state.sync_mode {
        SyncMode::Continuous(_) => {
            // Do nothing.
        },
        SyncMode::Blocks(ref bs) => {
            // MADR becomes end address, BCR becomes 0.
            let offset = bs.target_bamount_count + bs.target_bsize_count;
            let end_address = bs.current_address + offset as u32 * DATA_SIZE;
            madr.write_u32(end_address);
            bcr.write_u32(0);
        },
        SyncMode::LinkedList(_) => {
            // MADR becomes end code, BCR not touched.
            madr.write_u32(0x00FF_FFFF);
        },
        _ => panic!("Undefined sync mode"),
    }
}

pub(crate) fn handle_transfer(state: &State, controller_state: &mut ControllerState, channel_id: usize, ticks_remaining: &mut isize) -> Result<(), ()> {
    if state.dmac.dpcr.read_bitfield(DPCR_CHANNEL_ENABLE_BITFIELDS[channel_id]) == 0 {
        *ticks_remaining -= 1;
        return Ok(());
    }

    let transfer_state = get_transfer_state(controller_state, channel_id);

    if !transfer_state.started {
        *ticks_remaining -= 1;
        return Ok(());
    }

    state.bus_locked.store(true, Ordering::SeqCst);

    let mut finished = false;
    while *ticks_remaining > 0 {
        finished = if matches!(transfer_state.sync_mode, SyncMode::Continuous(_)) {
            let direction = transfer_state.direction;
            let step = transfer_state.step_direction;
            let continuous_state = transfer_state.sync_mode.as_continuous_mut().unwrap();
            let result = continuous::handle_transfer(state, continuous_state, channel_id, direction, step);
            process_continuous_result(channel_id, ticks_remaining, result)
        } else if matches!(transfer_state.sync_mode, SyncMode::Blocks(_)) {
            let direction = transfer_state.direction;
            let step = transfer_state.step_direction;
            let blocks_state = transfer_state.sync_mode.as_blocks_mut().unwrap();
            let result = blocks::handle_transfer(state, blocks_state, channel_id, direction, step);
            process_blocks_result(channel_id, ticks_remaining, result)
        } else if matches!(transfer_state.sync_mode, SyncMode::LinkedList(_)) {
            assert!(transfer_state.direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");
            let linked_list_state = transfer_state.sync_mode.as_linked_list_mut().unwrap();
            let result = linked_list::handle_transfer(state, linked_list_state, channel_id);
            process_linked_list_result(channel_id, ticks_remaining, result)
        } else {
            panic!("Invalid sync mode");
        }
        .map_err(|_| {
            state.bus_locked.store(false, Ordering::Release);
        })?;

        if finished {
            break;
        }
    }

    if finished {
        handle_transfer_finalization(state, transfer_state, channel_id);
        transfer_state.started = false;
        handle_irq_trigger(controller_state, channel_id);
        state.bus_locked.store(false, Ordering::Release);
    }

    Ok(())
}

fn process_continuous_result(channel_id: usize, ticks_remaining: &mut isize, result: Result<bool, ()>) -> Result<bool, ()> {
    let finished = result?;
    *ticks_remaining -= calculate_raw_channel_ticks(channel_id) as isize;
    Ok(finished)
}

fn process_blocks_result(channel_id: usize, ticks_remaining: &mut isize, result: Result<(bool, bool), ()>) -> Result<bool, ()> {
    let (finished, block_completed) = result?;

    *ticks_remaining -= calculate_raw_channel_ticks(channel_id) as isize;

    if block_completed {
        *ticks_remaining -= 16;
    }

    Ok(finished)
}

fn process_linked_list_result(channel_id: usize, ticks_remaining: &mut isize, result: Result<(bool, bool), ()>) -> Result<bool, ()> {
    let (finished, list_completed) = result?;

    *ticks_remaining -= calculate_raw_channel_ticks(channel_id) as isize;

    if list_completed {
        *ticks_remaining -= 16;
    }

    Ok(finished)
}

fn calculate_raw_channel_ticks(channel_id: usize) -> usize {
    match channel_id {
        0 => 1,
        1 => 1,
        2 => 1,
        // TODO: CDROM is variable based on cdrom_delay register.
        3 => 40,
        4 => 4,
        5 => 20,
        6 => 1,
        _ => unreachable!(),
    }
}
