use std::sync::atomic::{AtomicUsize, Ordering};
use log::{trace, warn};
use crate::State;
use crate::types::queue::Queue;
use crate::controllers::dmac::channel::*;
use crate::resources::dmac::*;
use crate::resources::dmac::debug::*;

static ENABLE_CHANNEL_STATE_CHANGE_TRACE: bool = false;
static ENABLE_CHANNEL_FIFO_HAZARD_READ_TRACE: bool = false;
static ENABLE_CHANNEL_FIFO_HAZARD_WRITE_TRACE: bool = false;

static mut TRANSFER_ID: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn transfer_start(state: &State, channel: usize) {
    if !ENABLE_CHANNEL_STATE_CHANGE_TRACE {
        return;
    }

    let transfer_id = TRANSFER_ID.fetch_add(1, Ordering::SeqCst);
    let chcr = &mut *get_chcr(state, channel);
    let madr = &mut *get_madr(state, channel);
    let bcr = &mut *get_bcr(state, channel);
    let sync_mode = get_sync_mode(chcr);
    let transfer_direction = get_transfer_direction(chcr);
    let transfer_state = &mut *get_transfer_state(state, channel);
    
    transfer_state.debug_state = Some(DebugState { transfer_id });

    trace!(
        "Starting transfer [{}] on channel {}, sync_mode = {:?}, direction = {:?}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
        transfer_id, channel, sync_mode, transfer_direction, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
    );
}

pub unsafe fn transfer_end(state: &State, channel: usize) {
    if !ENABLE_CHANNEL_STATE_CHANGE_TRACE {
        return;
    }

    let madr = &mut *get_madr(state, channel);
    let bcr = &mut *get_bcr(state, channel);
    let transfer_state = &mut *get_transfer_state(state, channel);

    let transfer_id = transfer_state.debug_state.unwrap().transfer_id;

    trace!(
        "Finished transfer [{}] on channel {}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
        transfer_id, channel, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
    );
}

pub fn trace_hazard_empty(fifo: &Queue<u32>) {
    if !ENABLE_CHANNEL_FIFO_HAZARD_READ_TRACE {
        return;
    }

    let debug_state = match fifo.debug_state {
        None => panic!("Queue debug state is required to trace hazards"),
        Some(ref d) => d,
    };

    warn!("DMAC: reading from {} but empty, trying again later", debug_state.identifier);
}

pub fn trace_hazard_full(fifo: &Queue<u32>) {
    if !ENABLE_CHANNEL_FIFO_HAZARD_WRITE_TRACE {
        return;
    }

    let debug_state = match fifo.debug_state {
        None => panic!("Queue debug state is required to trace hazards"),
        Some(ref d) => d,
    };

    warn!("DMAC: writing to {} but full, trying again later", debug_state.identifier);
}
