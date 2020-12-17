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
        types::{
            ControllerResult,
            State,
        },
    },
    types::bitfield::Bitfield,
};

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
            // log::warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before
            // sending/receiving next block");
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
    }

    if channel_id == 4 {
        log::debug!("SPU DMA transfer started");
    }

    get_transfer_flag(state, channel_id).store(true);
}

pub(crate) fn handle_transfer_finalization(state: &State, transfer_state: &mut TransferState, channel_id: usize) -> ControllerResult<()> {
    get_chcr(state, channel_id).update::<_, String>(|value| Ok(CHCR_STARTBUSY.insert_into(value, 0)))?;

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
    }

    if channel_id == 4 {
        log::debug!("SPU DMA transfer finished");
    }

    get_transfer_flag(state, channel_id).store(false);

    Ok(())
}

pub(crate) fn handle_transfer(state: &State, controller_state: &mut ControllerState, channel_id: usize, ticks_available: usize) -> ControllerResult<(usize, bool)> {
    let transfer_state = get_transfer_state(controller_state, channel_id);

    if (!transfer_state.enabled) || (!transfer_state.started) {
        return Ok((1, false));
    }

    state.bus_locked.store_barrier(true);

    let mut ticks_consumed = 0;
    while ticks_consumed < ticks_available {
        let direction = transfer_state.direction;
        let step = transfer_state.step_direction;

        let (cooloff_required, finished, block_completed) = match transfer_state.sync_mode {
            SyncMode::Continuous(ref mut continuous_state) => {
                let (cooloff_required, finished) = continuous::handle_transfer(state, continuous_state, channel_id, direction, step)?;
                (cooloff_required, finished, false)
            },
            SyncMode::Blocks(ref mut blocks_state) => blocks::handle_transfer(state, blocks_state, channel_id, direction, step)?,
            SyncMode::LinkedList(ref mut linked_list_state) => {
                if direction != TransferDirection::ToChannel {
                    return Err("Linked list transfers are ToChannel only".into());
                }

                linked_list::handle_transfer(state, linked_list_state, channel_id)?
            },
        };

        ticks_consumed += calculate_channel_ticks(channel_id, block_completed);

        if block_completed && !finished {
            state.bus_locked.store_barrier(false);
            return Ok((ticks_consumed, true));
        }

        if cooloff_required {
            state.bus_locked.store_barrier(false);
            return Ok((ticks_consumed, true));
        }

        if finished {
            handle_transfer_finalization(state, transfer_state, channel_id)?;
            transfer_state.started = false;
            handle_irq_trigger(state, controller_state, channel_id)?;
            state.bus_locked.store_barrier(false);
            break;
        }
    }

    Ok((ticks_consumed, false))
}

fn calculate_channel_ticks(channel_id: usize, block_completed: bool) -> usize {
    let mut ticks = calculate_raw_channel_ticks(channel_id);

    if block_completed {
        ticks += 16;
    }

    ticks
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
        _ => unreachable!("Invalid DMAC channel"),
    }
}
