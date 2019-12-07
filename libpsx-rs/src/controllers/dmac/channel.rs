use log::warn;
use crate::resources::Resources;
use crate::types::register::b32_register::B32Register;
use crate::types::bitfield::Bitfield;
use crate::types::fifo::Fifo;
use crate::resources::dmac::channel::*;
use crate::resources::dmac::register::*;
use crate::resources::dmac::*;
use crate::controllers::dmac::debug;

pub fn get_madr<'a, 'b>(resources: &'a mut Resources, channel: usize) -> &'b mut B32Register {
    let madr = match channel {
        0 => &mut resources.dmac.mdecin_madr,
        1 => &mut resources.dmac.mdecout_madr,
        2 => &mut resources.dmac.gpu_madr,
        3 => &mut resources.dmac.cdrom_madr,
        4 => &mut resources.dmac.spu_madr,
        5 => &mut resources.dmac.pio_madr,
        6 => &mut resources.dmac.otc_madr,
        _ => unreachable!("Invalid DMAC channel"),
    };

    unsafe {
        (madr as *mut B32Register).as_mut().unwrap()
    }
}

pub fn get_bcr<'a, 'b>(resources: &'a mut Resources, channel: usize) -> &'b mut B32Register {
    let bcr = match channel {
        0 => &mut resources.dmac.mdecin_bcr,
        1 => &mut resources.dmac.mdecout_bcr,
        2 => &mut resources.dmac.gpu_bcr,
        3 => &mut resources.dmac.cdrom_bcr,
        4 => &mut resources.dmac.spu_bcr,
        5 => &mut resources.dmac.pio_bcr,
        6 => &mut resources.dmac.otc_bcr,
        _ => unreachable!("Invalid DMAC channel"),
    };
    
    unsafe {
        (bcr as *mut B32Register).as_mut().unwrap()
    }
}

pub fn get_chcr<'a, 'b>(resources: &'a mut Resources, channel: usize) -> &'b mut Chcr {
    let chcr = match channel {
        0 => &mut resources.dmac.mdecin_chcr,
        1 => &mut resources.dmac.mdecout_chcr,
        2 => &mut resources.dmac.gpu_chcr,
        3 => &mut resources.dmac.cdrom_chcr,
        4 => &mut resources.dmac.spu_chcr,
        5 => &mut resources.dmac.pio_chcr,
        6 => &mut resources.dmac.otc_chcr.chcr,
        _ => unreachable!("Invalid DMAC channel"),
    };

    unsafe {
        (chcr as *mut Chcr).as_mut().unwrap()
    }
}

pub fn get_transfer_state<'a, 'b>(resources: &'a mut Resources, channel: usize) -> &'b mut TransferState {
    let transfer_state = match channel {
        0 => &mut resources.dmac.mdecin_transfer_state,
        1 => &mut resources.dmac.mdecout_transfer_state,
        2 => &mut resources.dmac.gpu_transfer_state,
        3 => &mut resources.dmac.cdrom_transfer_state,
        4 => &mut resources.dmac.spu_transfer_state,
        5 => &mut resources.dmac.pio_transfer_state,
        6 => &mut resources.dmac.otc_transfer_state,
        _ => unreachable!("Invalid DMAC channel"),
    };

    unsafe {
        (transfer_state as *mut TransferState).as_mut().unwrap()
    }
}

pub fn get_fifo<'a>(resources: &'a Resources, channel: usize, writing: bool) -> &'a Fifo<u32> {
    match channel {
        0 => unimplemented!("Unhandled DMAC channel 0"),
        1 => unimplemented!("Unhandled DMAC channel 1"),
        2 => {
            if writing {
                &resources.gpu.gpu1810.gp0
            } else {
                &resources.gpu.gpu1810.read
            }
        },
        3 => unimplemented!("Unhandled DMAC channel 3"),
        4 => unimplemented!("Unhandled DMAC channel 4"),
        5 => unimplemented!("Unhandled DMAC channel 5"),
        6 => panic!("DMAC channel 6 is not attached to a physical FIFO"),
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn pop_channel_data(resources: &Resources, channel: usize, madr: u32, last_transfer: bool) -> Result<u32, ()> {
    match channel {
        0..=5 => {
            let fifo = get_fifo(resources, channel, false);
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

pub fn push_channel_data(resources: &Resources, channel: usize, value: u32) -> Result<(), ()> {
    match channel {
        0..=5 => {
            let fifo = get_fifo(resources, channel, true);
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

pub fn get_transfer_direction(chcr: &Chcr) -> TransferDirection {
    match chcr.register.read_bitfield(CHCR_TRANSFER_DIRECTION) {
        0 => TransferDirection::FromChannel,
        1 => TransferDirection::ToChannel,
        _ => unreachable!("Invalid transfer direction"),
    }
}

pub fn get_step_direction(chcr: &Chcr) -> StepDirection {
    match chcr.register.read_bitfield(CHCR_MADR_STEP_DIRECTION) { 
        0 => StepDirection::Forwards,
        1 => StepDirection::Backwards,
        _ => unreachable!("Invalid step direction"),
    }
}

pub fn get_sync_mode(chcr: &Chcr) -> SyncMode {
    match chcr.register.read_bitfield(CHCR_SYNCMODE) {
        0 => SyncMode::Continuous,
        1 => SyncMode::Blocks,
        2 => SyncMode::LinkedList,
        _ => unreachable!("Invalid sync mode"),
    }
}

pub fn raise_irq(resources: &mut Resources, channel: usize) {
    let dicr = &mut resources.dmac.dicr;

    let _lock = dicr.mutex.lock();
    
    if dicr.register.read_bitfield(DICR_IRQ_ENABLE_BITFIELDS[channel]) != 0 {
        dicr.register.write_bitfield(DICR_IRQ_FLAG_BITFIELDS[channel], 1);
    }
}

pub fn initialize_transfer_state(transfer_state: &mut TransferState, chcr: &Chcr, madr: &B32Register, bcr: &B32Register) {
    let bcr_calculate = |v| {
        if v == 0 {
            0x1_0000
        } else {
            v
        }
    };

    let address = madr.read_bitfield(Bitfield::new(0, 24));
    let sync_mode = get_sync_mode(chcr);
    let bs_count = bcr_calculate(bcr.read_bitfield(BCR_BLOCKSIZE) as usize);
    let ba_count = bcr_calculate(bcr.read_bitfield(BCR_BLOCKAMOUNT) as usize);

    *transfer_state = TransferState::reset();

    match sync_mode {
        SyncMode::Continuous => {
            transfer_state.sync_mode_state = SyncModeState::Continuous(
                ContinuousState {
                    current_address: address,
                    current_count: 0,
                    target_count: bs_count,
                }
            );
        },
        SyncMode::Blocks => {
            warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending next block");

            transfer_state.sync_mode_state = SyncModeState::Blocks(
                BlocksState {
                    current_address: address,
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
                    current_header_address: 0,
                    next_header_address: address,
                    target_count: 0,
                    current_count: 0,
                }
            );
        },
    }

    transfer_state.started = true;
}
