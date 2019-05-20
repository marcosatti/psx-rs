use crate::State;
use crate::controllers::spu::voice::*;
use crate::resources::spu::*;
use crate::resources::spu::register::*;

pub unsafe fn handle_current_transfer_address(state: &State) {
    let resources = &mut *state.resources;
    let control = &resources.spu.control;
    let data_transfer_address = &mut resources.spu.data_transfer_address;
    let current_transfer_adderss = &mut resources.spu.current_transfer_address;

    if data_transfer_address.write_latch {
        if get_transfer_mode(control) != TransferMode::Stop {
            panic!("A write to the data transfer register happened while a transfer was in progress - probably bad");
        }

        *current_transfer_adderss = data_transfer_address.register.read_u16() as u32 * 8;
        data_transfer_address.write_latch = false;
    }
}

pub unsafe fn handle_new_transfer_initialization(state: &State) {
    let resources = &mut *state.resources;
    let control = &resources.spu.control;
    let stat = &mut resources.spu.stat;
    let current_transfer_mode = &mut resources.spu.current_transfer_mode;

    let new_transfer_mode = get_transfer_mode(control);
    if new_transfer_mode != TransferMode::Stop {
        *current_transfer_mode = new_transfer_mode;
        stat.write_bitfield(STAT_DATA_BUSY_FLAG, 1);
    }

    let transfer_mode_raw = control.read_bitfield(CONTROL_TRANSFER_MODE);
    stat.write_bitfield(STAT_TRANSFER_MODE, transfer_mode_raw);
}

pub unsafe fn handle_manual_write_transfer(state: &State) {
    let resources = &mut *state.resources;
    let control = &mut resources.spu.control;
    let stat = &mut resources.spu.stat;
    let memory = &mut resources.spu.memory;
    let current_transfer_mode = &mut resources.spu.current_transfer_mode;
    let current_transfer_address = &mut resources.spu.current_transfer_address;

    let data_transfer_control = &resources.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    let _lock = resources.spu.data_fifo.lock.lock();
    let fifo = &mut resources.spu.data_fifo.fifo;

    match fifo.pop_front() {
        Some(value) => {
            memory.write_u16(*current_transfer_address as usize, value);
            *current_transfer_address += 2;
            *current_transfer_address &= 0x7FFFF;
        },
        None => {
            control.write_bitfield(CONTROL_TRANSFER_MODE, 0);
            stat.write_bitfield(STAT_DATA_BUSY_FLAG, 0);
            stat.write_bitfield(STAT_TRANSFER_MODE, 0);
            *current_transfer_mode = TransferMode::Stop;
        },
    }
}

pub unsafe fn handle_dma_write_transfer(state: &State) {
    let resources = &mut *state.resources;
    let data_transfer_control = &resources.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    unimplemented!("DmaWrite transfer mode not implemented");
}

pub unsafe fn handle_dma_read_transfer(state: &State) {
    let resources = &mut *state.resources;
    let data_transfer_control = &resources.spu.data_transfer_control;
    if data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    unimplemented!("DmaRead transfer mode not implemented");
}
