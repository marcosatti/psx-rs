use crate::{
    system::r3000::{
        cp0::types::ControllerState as Cp0ControllerState,
        cp2::types::ControllerState as Cp2ControllerState,
        types::ControllerState,
    },
    types::{
        bitfield::Bitfield,
        mips1::register::Register,
    },
};

pub(crate) fn handle_zero(state: &mut ControllerState) {
    state.gpr[0].write_u32(0);
}

pub(crate) fn handle_cp2_push_sz(state: &mut Cp2ControllerState) {
    let sz1_value = state.gd[17].read_u32();
    let sz2_value = state.gd[18].read_u32();
    let sz3_value = state.gd[19].read_u32();
    state.gd[16].write_u32(sz1_value); // SZ0 = SZ1
    state.gd[17].write_u32(sz2_value); // SZ1 = SZ2
    state.gd[18].write_u32(sz3_value); // SZ2 = SZ3
}

pub(crate) fn handle_cp2_push_sxy(state: &mut Cp2ControllerState) {
    let sxy1_value = state.gd[13].read_u32();
    let sxy2_value = state.gd[14].read_u32();
    state.gd[12].write_u32(sxy1_value); // SXY0 = SXY1
    state.gd[13].write_u32(sxy2_value); // SXY1 = SXY2
}

pub(crate) fn handle_cp2_sxyp_mirror(state: &mut Cp2ControllerState) {
    let value = state.gd[14].read_u32();
    state.gd[15].write_u32(value);
}

pub(crate) fn handle_cp2_sxyp_write(state: &mut Cp2ControllerState, register_index: usize) {
    if register_index == 15 {
        handle_cp2_push_sxy(state);
        let value = state.gd[15].read_u32();
        state.gd[14].write_u32(value); // SXY2 = SXYP
    }
}

pub(crate) fn handle_cp2_flag_reset(state: &mut Cp2ControllerState) {
    state.gc[31].write_u32(0);
}

pub(crate) fn handle_cp2_flag_error_bit(state: &mut Cp2ControllerState) {
    let set1 = state.gc[31].read_bitfield(Bitfield::new(13, 6)) > 0;
    let set2 = state.gc[31].read_bitfield(Bitfield::new(23, 8)) > 0;
    if set1 || set2 {
        state.gc[31].write_bitfield(Bitfield::new(31, 1), 1);
    }
}

pub(crate) fn handle_cp2_push_rgb(state: &mut Cp2ControllerState) {
    let rgb1_value = state.gd[21].read_u32();
    let rgb2_value = state.gd[22].read_u32();
    state.gd[20].write_u32(rgb1_value); // RGB0 = RGB1
    state.gd[21].write_u32(rgb2_value); // RGB1 = RGB2
}

pub(crate) fn get_cp0_register(state: &mut Cp0ControllerState, register_id: usize) -> &mut Register {
    match register_id {
        3 => &mut state.bpc,
        5 => &mut state.bda,
        6 => &mut state.jump_dest,
        7 => &mut state.dcic,
        9 => &mut state.bdam,
        11 => &mut state.bpcm,
        12 => &mut state.status,
        13 => &mut state.cause,
        14 => &mut state.epc,
        15 => &mut state.prid,
        _ => unimplemented!(),
    }
}
