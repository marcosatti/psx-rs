use crate::{
    system::{
        dmac::types::*,
        types::State,
    },
    types::{
        flag::Flag,
        memory::*,
    },
};

pub(crate) fn get_madr(state: &State, channel_id: usize) -> &B32LevelRegister {
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

pub(crate) fn get_bcr(state: &State, channel_id: usize) -> &B32LevelRegister {
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

pub(crate) fn get_chcr(state: &State, channel_id: usize) -> &B32EdgeRegister {
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

pub(crate) fn get_transfer_flag(state: &State, channel_id: usize) -> &Flag {
    match channel_id {
        0 => &state.dmac.mdecin_transfer_flag,
        1 => &state.dmac.mdecout_transfer_flag,
        2 => &state.dmac.gpu_transfer_flag,
        3 => &state.dmac.cdrom_transfer_flag,
        4 => &state.dmac.spu_transfer_flag,
        5 => &state.dmac.pio_transfer_flag,
        6 => &state.dmac.otc_transfer_flag,
        _ => unreachable!("Invalid DMAC channel"),
    }
}

pub(crate) fn get_transfer_state(controller_state: &mut ControllerState, channel_id: usize) -> &mut TransferState {
    match channel_id {
        0 => &mut controller_state.mdecin_transfer_state,
        1 => &mut controller_state.mdecout_transfer_state,
        2 => &mut controller_state.gpu_transfer_state,
        3 => &mut controller_state.cdrom_transfer_state,
        4 => &mut controller_state.spu_transfer_state,
        5 => &mut controller_state.pio_transfer_state,
        6 => &mut controller_state.otc_transfer_state,
        _ => unreachable!("Invalid DMAC channel"),
    }
}
