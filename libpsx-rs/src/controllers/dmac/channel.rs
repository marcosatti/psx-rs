use crate::resources::Resources;
use crate::types::register::b32_register::B32Register;
use crate::types::bitfield::Bitfield;
use crate::types::fifo::Fifo;
use crate::resources::dmac::channel::*;
use crate::resources::dmac::*;
use crate::controllers::dmac::debug;
    
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferDirection {
    FromChannel,
    ToChannel,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StepDirection {
    Forwards,
    Backwards,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyncMode {
    Continuous,
    Blocks,
    LinkedList,
}

pub unsafe fn get_madr(resources: &mut Resources, channel: usize) -> *mut B32Register {
    match channel {
        0 => &mut resources.dmac.mdecin_madr,
        1 => &mut resources.dmac.mdecout_madr,
        2 => &mut resources.dmac.gpu_madr,
        3 => &mut resources.dmac.cdrom_madr,
        4 => &mut resources.dmac.spu_madr,
        5 => &mut resources.dmac.pio_madr,
        6 => &mut resources.dmac.otc_madr,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn get_bcr(resources: &mut Resources, channel: usize) -> *mut B32Register {
    match channel {
        0 => &mut resources.dmac.mdecin_bcr,
        1 => &mut resources.dmac.mdecout_bcr,
        2 => &mut resources.dmac.gpu_bcr,
        3 => &mut resources.dmac.cdrom_bcr,
        4 => &mut resources.dmac.spu_bcr,
        5 => &mut resources.dmac.pio_bcr,
        6 => &mut resources.dmac.otc_bcr,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn get_chcr(resources: &mut Resources, channel: usize) -> *mut B32Register {
    match channel {
        0 => &mut resources.dmac.mdecin_chcr.register,
        1 => &mut resources.dmac.mdecout_chcr.register,
        2 => &mut resources.dmac.gpu_chcr.register,
        3 => &mut resources.dmac.cdrom_chcr.register,
        4 => &mut resources.dmac.spu_chcr.register,
        5 => &mut resources.dmac.pio_chcr.register,
        6 => &mut resources.dmac.otc_chcr.chcr.register,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn get_transfer_state(resources: &mut Resources, channel: usize) -> *mut TransferState {
    match channel {
        0 => &mut resources.dmac.mdecin_transfer_state,
        1 => &mut resources.dmac.mdecout_transfer_state,
        2 => &mut resources.dmac.gpu_transfer_state,
        3 => &mut resources.dmac.cdrom_transfer_state,
        4 => &mut resources.dmac.spu_transfer_state,
        5 => &mut resources.dmac.pio_transfer_state,
        6 => &mut resources.dmac.otc_transfer_state,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn get_fifo<'a>(resources: &'a Resources, channel: usize) -> &'a Fifo<u32> {
    match channel {
        0 => unimplemented!("Unhandled DMAC channel 0"),
        1 => unimplemented!("Unhandled DMAC channel 1"),
        2 => &resources.gpu.gpu1810.gp0,
        3 => unimplemented!("Unhandled DMAC channel 3"),
        4 => unimplemented!("Unhandled DMAC channel 4"),
        5 => unimplemented!("Unhandled DMAC channel 5"),
        6 => panic!("DMAC channel 6 is not attached to a physical FIFO"),
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn pop_channel_data(resources: &mut Resources, channel: usize, madr: u32, last_transfer: bool) -> Result<u32, ()> {
    match channel {
        0..=5 => {
            let fifo = get_fifo(resources, channel);
            let result = fifo.read_one();

            if result.is_err() {
                debug::trace_hazard_empty(fifo);
            }

            result
        },
        6 => {
            Ok(if !last_transfer { (madr - 4) & 0x00FF_FFFF } else { 0x00FF_FFFF })
        },
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub unsafe fn push_channel_data(resources: &mut Resources, channel: usize, value: u32) -> Result<(), ()> {
    match channel {
        0..=5 => {
            let fifo = get_fifo(resources, channel);
            let result = fifo.write_one(value);
            
            if result.is_err() {
                debug::trace_hazard_full(fifo);
            }

            result
        },
        6 => panic!("Channel 6 cannot recieve data (OTC)"),
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn get_transfer_direction(chcr: &B32Register) -> TransferDirection {
    match chcr.read_bitfield(CHCR_TRANSFER_DIRECTION) {
        0 => TransferDirection::FromChannel,
        1 => TransferDirection::ToChannel,
        _ => unreachable!("Invalid transfer direction"),
    }
}

pub fn get_step_direction(chcr: &B32Register) -> StepDirection {
    match chcr.read_bitfield(CHCR_MADR_STEP_DIRECTION) { 
        0 => StepDirection::Forwards,
        1 => StepDirection::Backwards,
        _ => unreachable!("Invalid step direction"),
    }
}

pub fn get_sync_mode(chcr: &B32Register) -> SyncMode {
    match chcr.read_bitfield(CHCR_SYNCMODE) {
        0 => SyncMode::Continuous,
        1 => SyncMode::Blocks,
        2 => SyncMode::LinkedList,
        _ => unreachable!("Invalid sync mode"),
    }
}

pub fn initialize_transfer(transfer_state: &mut TransferState, sync_mode: SyncMode, madr: &B32Register, bcr: &B32Register) {
    *transfer_state = TransferState::reset();

    let mut madr_value = madr.read_u32();
    madr_value = Bitfield::new(0, 24).extract_from(madr_value);
    let mut bs_count = bcr.read_bitfield(BCR_BLOCKSIZE) as usize;
    bs_count = if bs_count == 0 { 0x1_0000 } else { bs_count };
    let mut ba_count = bcr.read_bitfield(BCR_BLOCKAMOUNT) as usize;
    ba_count = if ba_count == 0 { 0x1_0000 } else { ba_count };

    match sync_mode {
        SyncMode::Continuous => {
            transfer_state.sync_mode_state = SyncModeState::Continuous(
                ContinuousState {
                    current_address: madr_value,
                    current_count: 0,
                    target_count: bs_count,
                }
            );
        },
        SyncMode::Blocks => {
            transfer_state.sync_mode_state = SyncModeState::Blocks(
                BlocksState {
                    current_address: madr_value,
                    current_bsize_count: 0,
                    target_bsize_count: bs_count,
                    current_bamount_count: 0,
                    target_bamount_count: ba_count,
                }
            );
        },
        SyncMode::LinkedList => {
            transfer_state.sync_mode_state = SyncModeState::LinkedList(
                LinkedListState {
                    current_address: 0,
                    next_address: madr_value,
                    target_count: 0,
                    current_count: 0,
                }
            );
        },
    }

    transfer_state.started = true;
}
