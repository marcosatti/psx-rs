use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt::{Display, UpperHex};
use log::trace;
use crate::system::Resources;
use crate::types::fifo::Fifo;
use crate::controllers::dmac::channel::*;
use crate::system::dmac::*;
use crate::system::dmac::debug::*;

const ENABLE_CHANNEL_STATE_CHANGE_TRACE: bool = false;
const ENABLE_CHANNEL_FIFO_HAZARD_READ_TRACE: bool = false;
const ENABLE_CHANNEL_FIFO_HAZARD_WRITE_TRACE: bool = false;
const ENABLE_LINKED_LIST_NULL_HEADER_TRACE: bool = true;

static TRANSFER_ID: AtomicUsize = AtomicUsize::new(0);

pub fn transfer_start(resources: &mut Resources, channel: usize) {
    if !ENABLE_CHANNEL_STATE_CHANGE_TRACE {
        return;
    }

    let transfer_id = TRANSFER_ID.fetch_add(1, Ordering::Relaxed);

    let chcr = get_chcr(resources, channel);
    let madr = get_madr(resources, channel);
    let bcr = get_bcr(resources, channel);
    let sync_mode = get_sync_mode(chcr);
    let transfer_direction = get_transfer_direction(chcr);
    let transfer_state = get_transfer_state(resources, channel);
    
    transfer_state.debug_state = Some(DebugState { transfer_id });

    trace!(
        "Starting transfer [{}] on channel {}, sync_mode = {:?}, direction = {:?}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
        transfer_id, channel, sync_mode, transfer_direction, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
    );
}

pub fn transfer_end(resources: &mut Resources, channel: usize) {
    if !ENABLE_CHANNEL_STATE_CHANGE_TRACE {
        return;
    }

    let madr = get_madr(resources, channel);
    let bcr = get_bcr(resources, channel);
    let transfer_state = get_transfer_state(resources, channel);

    let transfer_id = transfer_state.debug_state.unwrap().transfer_id;

    trace!(
        "Finished transfer [{}] on channel {}, bs (raw) = {}, ba (raw) = {}, madr (raw) = 0x{:0X}", 
        transfer_id, channel, bcr.read_bitfield(BCR_BLOCKSIZE), bcr.read_bitfield(BCR_BLOCKAMOUNT), madr.read_u32()
    );
}

pub fn trace_hazard_empty<T: Copy + Default + Display + UpperHex>(fifo: &Fifo<T>) {
    if !ENABLE_CHANNEL_FIFO_HAZARD_READ_TRACE {
        return;
    }

    let debug_state = match fifo.debug_state {
        None => panic!("Fifo debug state is required to trace hazards"),
        Some(ref d) => d,
    };

    trace!("DMAC: reading from {} but empty, trying again later", debug_state.identifier);
}

pub fn trace_hazard_full(fifo: &Fifo<u32>) {
    if !ENABLE_CHANNEL_FIFO_HAZARD_WRITE_TRACE {
        return;
    }

    let debug_state = match fifo.debug_state {
        None => panic!("Fifo debug state is required to trace hazards"),
        Some(ref d) => d,
    };

    trace!("DMAC: writing to {} but full, trying again later", debug_state.identifier);
}

pub fn trace_dmac(resources: &Resources, only_enabled: bool) {
    let dpcr = resources.dmac.dpcr.read_u32();
    for (name, bitfield) in DMA_CHANNEL_NAMES.iter().zip(DPCR_CHANNEL_ENABLE_BITFIELDS.iter()) {
        let dpcr_value = bitfield.extract_from(dpcr) != 0;

        if only_enabled && !dpcr_value {
            continue;
        }

        trace!("DMAC DPCR [{}]: dma enabled = {}", name, dpcr_value);
    }

    let dicr = resources.dmac.dicr.register.read_u32();
    let dicr_irq_master_enable_value = DICR_IRQ_MASTER_ENABLE.extract_from(dicr) != 0;
    trace!("DMAC DICR: master enable = {}", dicr_irq_master_enable_value);
    let dicr_irq_force_value = DICR_IRQ_FORCE.extract_from(dicr) != 0;
    trace!("DMAC DICR: irq force = {}", dicr_irq_force_value);
    for (name, (enable_bitfield, flag_bitfield)) in DMA_CHANNEL_NAMES.iter().zip(DICR_IRQ_ENABLE_BITFIELDS.iter().zip(DICR_IRQ_FLAG_BITFIELDS.iter())) {
        let dicr_enable_value = enable_bitfield.extract_from(dicr) != 0; 
        let dicr_flag_value = flag_bitfield.extract_from(dicr) != 0; 

        if only_enabled && !dicr_enable_value {
            continue;
        }

        trace!("DMAC DICR [{}]: irq enabled = {}, irq flag = {}", name, dicr_enable_value, dicr_flag_value);
    }
    let dicr_irq_master_flag_value = DICR_IRQ_MASTER_FLAG.extract_from(dicr) != 0;
    trace!("DMAC DICR: master flag = {}", dicr_irq_master_flag_value);
}

pub fn trace_linked_list_null_header(header_address: u32) {
    if !ENABLE_LINKED_LIST_NULL_HEADER_TRACE {
        return;
    }

    trace!("Null linked list transfer header value (address 0x{:08X}) - CPU is probably too slow (or bad timing)!", header_address);
}
