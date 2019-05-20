use std::sync::atomic::{AtomicUsize, Ordering};
use log::debug;
use crate::State;
use crate::controllers::dmac::channel::*;
use crate::resources::dmac::*;
use crate::resources::dmac::channel::*;

static ENABLE_TRACE: bool = false;
static mut TRANSFER_ID: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn transfer_start(state: &State, channel: usize) {
    let transfer_id = TRANSFER_ID.fetch_add(1, Ordering::SeqCst);
    let chcr = &mut *get_chcr(state, channel);
    let madr = &mut *get_madr(state, channel);
    let bcr = &mut *get_bcr(state, channel);
    let sync_mode = get_sync_mode(chcr);
    let transfer_direction = get_transfer_direction(chcr);
    let transfer_state = &mut *get_transfer_state(state, channel);
    
    transfer_state.debug_state = Some(DebugState { transfer_id });
    
    if ENABLE_TRACE {
        debug!(
            "Starting transfer [{}] on channel {}, sync_mode = {:?}, direction = {:?}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
            transfer_id, channel, sync_mode, transfer_direction, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
        );
    }
}

pub unsafe fn transfer_end(state: &State, channel: usize) {
    let madr = &mut *get_madr(state, channel);
    let bcr = &mut *get_bcr(state, channel);
    let transfer_state = &mut *get_transfer_state(state, channel);

    let transfer_id = transfer_state.debug_state.unwrap().transfer_id;

    if ENABLE_TRACE {
        debug!(
            "Finished transfer [{}] on channel {}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
            transfer_id, channel, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
        );
    }
}