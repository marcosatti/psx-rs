use crate::{
    system::{
        dmac::{
            constants::*,
            controllers::debug,
            types::*,
        },
        types::State,
    },
    types::{
        bitfield::Bitfield,
        memory::*,
    },
};
use log::warn;

pub fn get_madr(state: &State, channel_id: usize) -> &B32LevelRegister {
    match channel_id {
        0 => &state.dmac.mdecin_madr,
        1 => &state.dmac.mdecout_madr,
        2 => &state.dmac.gpu_madr,
        3 => &state.dmac.cdrom_madr,
        4 => &state.dmac.spu_madr,
        5 => &state.dmac.pio_madr,
        6 => &state.dmac.otc_madr,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn get_bcr(state: &State, channel_id: usize) -> &B32LevelRegister {
    match channel_id {
        0 => &state.dmac.mdecin_bcr,
        1 => &state.dmac.mdecout_bcr,
        2 => &state.dmac.gpu_bcr,
        3 => &state.dmac.cdrom_bcr,
        4 => &state.dmac.spu_bcr,
        5 => &state.dmac.pio_bcr,
        6 => &state.dmac.otc_bcr,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn get_chcr(state: &State, channel_id: usize) -> &Chcr {
    match channel_id {
        0 => &state.dmac.mdecin_chcr,
        1 => &state.dmac.mdecout_chcr,
        2 => &state.dmac.gpu_chcr,
        3 => &state.dmac.cdrom_chcr,
        4 => &state.dmac.spu_chcr,
        5 => &state.dmac.pio_chcr,
        6 => &state.dmac.otc_chcr,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn get_transfer_state(state: &mut ControllerState, channel_id: usize) -> &mut TransferState {
    match channel_id {
        0 => &mut state.mdecin_transfer_state,
        1 => &mut state.mdecout_transfer_state,
        2 => &mut state.gpu_transfer_state,
        3 => &mut state.cdrom_transfer_state,
        4 => &mut state.spu_transfer_state,
        5 => &mut state.pio_transfer_state,
        6 => &mut state.otc_transfer_state,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

fn get_otc_value(madr_value: u32, last_transfer: bool) -> u32 {
    if !last_transfer {
        (madr_value - 4) & 0x00FF_FFFF
    } else {
        0x00FF_FFFF
    }
}

pub fn pop_channel_data(state: &State, channel_id: usize, madr: u32, last_transfer: bool) -> Result<u32, ()> {
    match channel_id {
        0 => unimplemented!("Unhandled DMAC channel 0"),
        1 => unimplemented!("Unhandled DMAC channel 1"),
        2 => {
            let fifo = &state.gpu.read;
            let handle_error = |e| {
                debug::trace_hazard_empty(fifo);
                e
            };
            fifo.read_one().map_err(handle_error)
        },
        3 => {
            let fifo = &state.cdrom.data;
            if fifo.read_available() < 4 {
                debug::trace_hazard_empty(fifo);
                return Err(());
            }
            let result1 = fifo.read_one().unwrap();
            let result2 = fifo.read_one().unwrap();
            let result3 = fifo.read_one().unwrap();
            let result4 = fifo.read_one().unwrap();
            Ok(u32::from_le_bytes([result1, result2, result3, result4]))
        },
        4 => unimplemented!("Unhandled DMAC channel 4"),
        5 => unimplemented!("Unhandled DMAC channel 5"),
        6 => Ok(get_otc_value(madr, last_transfer)),
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub fn push_channel_data(state: &State, channel_id: usize, value: u32) -> Result<(), ()> {
    match channel_id {
        0 => unimplemented!("Unhandled DMAC channel 0"),
        1 => unimplemented!("Unhandled DMAC channel 1"),
        2 => {
            let fifo = &state.gpu.gp0;
            let handle_error = |e| {
                debug::trace_hazard_full(fifo);
                e
            };
            fifo.write_one(value).map_err(handle_error)
        },
        3 => unimplemented!("Unhandled DMAC channel 3"),
        4 => unimplemented!("Unhandled DMAC channel 4"),
        5 => unimplemented!("Unhandled DMAC channel 5"),
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

pub fn raise_irq(state: &State, channel_id: usize) {
    let dicr = &state.dmac.dicr;

    let _lock = dicr.mutex.lock();

    if dicr.register.read_bitfield(DICR_IRQ_ENABLE_BITFIELDS[channel_id]) != 0 {
        dicr.register.write_bitfield(DICR_IRQ_FLAG_BITFIELDS[channel_id], 1);
    }
}

pub fn initialize_transfer_state(transfer_state: &mut TransferState, chcr: &Chcr, madr: &B32LevelRegister, bcr: &B32LevelRegister) {
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
            transfer_state.sync_mode_state = SyncModeState::Continuous(ContinuousState {
                current_address: address,
                current_count: 0,
                target_count: bs_count,
            });
        },
        SyncMode::Blocks => {
            warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending/receiving next block");

            let blocks_state = BlocksState {
                current_address: address,
                current_bsize_count: 0,
                target_bsize_count: bs_count,
                current_bamount_count: 0,
                target_bamount_count: ba_count,
            };

            transfer_state.sync_mode_state = SyncModeState::Blocks(blocks_state);
        },
        SyncMode::LinkedList => {
            transfer_state.sync_mode_state = SyncModeState::LinkedList(LinkedListState {
                current_header_address: 0,
                next_header_address: address,
                target_count: 0,
                current_count: 0,
            });
        },
    }

    transfer_state.started = true;
}
