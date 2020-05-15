pub mod continuous;
pub mod blocks;
pub mod linked_list;
pub mod fifo;

use crate::system::{
    dmac::{
        constants::*,
        controllers::{
            channel::*,
            interrupt::*,
        },
        types::*,
    },
    types::State,
};
use crate::types::bitfield::Bitfield;
use std::sync::atomic::Ordering;

pub fn handle_transfer_initialization(state: &State, transfer_state: &mut TransferState, channel_id: usize) {
    const ADDRESS: Bitfield = Bitfield::new(0, 24);

    transfer_state.delay_cycles = match channel_id {
        2 => 0x80,
        _ => 0x0,
    };

    let bcr_calculate = |v| {
        if v == 0 {
            0x1_0000
        } else {
            v
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
            log::warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending/receiving next block");
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

pub fn handle_transfer_finalization(state: &State, transfer_state: &mut TransferState, channel_id: usize) {    
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

pub fn handle_transfer(state: &State, controller_state: &mut ControllerState, channel_id: usize) -> Result<usize, ()> {
    if state.dmac.dpcr.read_bitfield(DPCR_CHANNEL_ENABLE_BITFIELDS[channel_id]) == 0 {
        return Ok(0);
    }

    let transfer_state = get_transfer_state(controller_state, channel_id);

    if !transfer_state.started {
        return Ok(0);
    }

    if transfer_state.delay_cycles > 0 {
        transfer_state.delay_cycles -= 1;
        return Ok(0);
    }

    state.bus_locked.store(true, Ordering::SeqCst);

    let mut count = 0;
    let mut finished = false;
    for _ in 0..16 {
        let result = match transfer_state.sync_mode {
            SyncMode::Continuous(ref mut cs) => {
                let direction = transfer_state.direction;
                let step = transfer_state.step_direction;
                continuous::handle_transfer(state, cs, channel_id, direction, step)
            },
            SyncMode::Blocks(ref mut bs) => {
                let direction = transfer_state.direction;
                let step = transfer_state.step_direction;
                blocks::handle_transfer(state, bs, channel_id, direction, step)
            },
            SyncMode::LinkedList(ref mut lls) => {
                assert!(transfer_state.direction == TransferDirection::ToChannel, "Linked list transfers are ToChannel only");
                linked_list::handle_transfer(state, lls, channel_id)
            },
            _ => panic!("Undefined sync mode"),
        };

        match result {
            Ok(false) => {
                count += 1;
            },
            Ok(true) => {
                count += 1;
                finished = true;
                break;
            },
            Err(()) => {
                state.bus_locked.store(false, Ordering::Release);
                return Err(());
            },
        }
    }

    if finished {
        handle_transfer_finalization(state, transfer_state, channel_id);
        transfer_state.started = false;
        handle_irq_trigger(controller_state, channel_id);
    }

    state.bus_locked.store(false, Ordering::Release);

    Ok(count)
}
