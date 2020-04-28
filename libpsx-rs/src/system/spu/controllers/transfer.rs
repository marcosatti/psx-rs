use crate::system::{
    spu::{
        constants::*,
        controllers::voice::*,
        types::*,
    },
    types::State,
};
use std::sync::atomic::Ordering;

pub fn handle_transfer(state: &State, spu_state: &mut ControllerState) {
    let current_transfer_mode = spu_state.current_transfer_mode;

    handle_current_transfer_address(state, spu_state);

    match current_transfer_mode {
        TransferMode::Stop => {
            handle_new_transfer_initialization(state, spu_state);
        },
        TransferMode::ManualWrite => {
            handle_manual_write_transfer(state, spu_state);
        },
        TransferMode::DmaWrite => {
            handle_dma_write_transfer(state);
        },
        TransferMode::DmaRead => {
            handle_dma_read_transfer(state);
        },
    }
}

fn handle_current_transfer_address(state: &State, spu_state: &mut ControllerState) {
    let control = &state.spu.control;
    let data_transfer_address = &state.spu.data_transfer_address;
    let current_transfer_adderss = &mut spu_state.current_transfer_address;

    if data_transfer_address.write_latch.load(Ordering::Acquire) {
        if get_transfer_mode(control) != TransferMode::Stop {
            panic!("A write to the data transfer register happened while a transfer was in progress - probably bad");
        }

        *current_transfer_adderss = data_transfer_address.register.read_u16() as u32 * 8;
        data_transfer_address.write_latch.store(false, Ordering::Release);
    }
}

fn handle_new_transfer_initialization(state: &State, spu_state: &mut ControllerState) {
    let control = &state.spu.control;
    let stat = &mut state.spu.stat;
    let current_transfer_mode = &mut spu_state.current_transfer_mode;

    let new_transfer_mode = get_transfer_mode(control);
    if new_transfer_mode != TransferMode::Stop {
        *current_transfer_mode = new_transfer_mode;
        stat.write_bitfield(STAT_DATA_BUSY_FLAG, 1);
    }

    let transfer_mode_raw = control.read_bitfield(CONTROL_TRANSFER_MODE);
    stat.write_bitfield(STAT_TRANSFER_MODE, transfer_mode_raw);
}

fn handle_manual_write_transfer(state: &State, spu_state: &mut ControllerState) {
    let control = &mut state.spu.control;
    let stat = &mut state.spu.stat;
    let memory = &mut spu_state.memory;
    let current_transfer_mode = &mut spu_state.current_transfer_mode;
    let current_transfer_address = &mut spu_state.current_transfer_address;

    let data_transfer_control = &state.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    let fifo = &state.spu.data_fifo;

    match fifo.read_one() {
        Ok(value) => {
            let bytes = u16::to_le_bytes(value);
            memory[*current_transfer_address as usize] = bytes[0];
            memory[*current_transfer_address as usize + 1] = bytes[1];
            *current_transfer_address += 2;
            *current_transfer_address &= 0x7FFFF;
        },
        Err(_) => {
            control.write_bitfield(CONTROL_TRANSFER_MODE, 0);
            stat.write_bitfield(STAT_DATA_BUSY_FLAG, 0);
            stat.write_bitfield(STAT_TRANSFER_MODE, 0);
            *current_transfer_mode = TransferMode::Stop;
        },
    }
}

fn handle_dma_write_transfer(state: &State) {
    let data_transfer_control = &state.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    unimplemented!("DmaWrite transfer mode not implemented");
}

fn handle_dma_read_transfer(state: &State) {
    let data_transfer_control = &state.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    unimplemented!("DmaRead transfer mode not implemented");
}
