use crate::system::Resources;
use crate::types::bitfield::Bitfield;

pub fn handle_zero(resources: &mut Resources) {
    resources.r3000.gpr[0].write_u32(0);
}

pub fn handle_cp2_push_sz(resources: &mut Resources) {
    let sz1_value = resources.r3000.cp2.gd[17].read_u32();
    let sz2_value = resources.r3000.cp2.gd[18].read_u32();
    let sz3_value = resources.r3000.cp2.gd[19].read_u32();
    resources.r3000.cp2.gd[16].write_u32(sz1_value); // SZ0 = SZ1
    resources.r3000.cp2.gd[17].write_u32(sz2_value); // SZ1 = SZ2
    resources.r3000.cp2.gd[18].write_u32(sz3_value); // SZ2 = SZ3
}

pub fn handle_cp2_push_sxy(resources: &mut Resources) {
    let sxy1_value = resources.r3000.cp2.gd[13].read_u32();
    let sxy2_value = resources.r3000.cp2.gd[14].read_u32();
    resources.r3000.cp2.gd[12].write_u32(sxy1_value); // SXY0 = SXY1
    resources.r3000.cp2.gd[13].write_u32(sxy2_value); // SXY1 = SXY2
}

pub fn handle_cp2_sxyp_mirror(resources: &mut Resources) {
    let value = resources.r3000.cp2.gd[14].read_u32();
    resources.r3000.cp2.gd[15].write_u32(value);
}

pub fn handle_cp2_sxyp_write(resources: &mut Resources, register_index: usize) {
    if register_index == 15 {
        handle_cp2_push_sxy(resources);
        let value = resources.r3000.cp2.gd[15].read_u32();
        resources.r3000.cp2.gd[14].write_u32(value); // SXY2 = SXYP
    }
}

pub fn handle_cp2_flag_reset(resources: &mut Resources) {
    resources.r3000.cp2.gc[31].write_u32(0);
}

pub fn handle_cp2_flag_error_bit(resources: &mut Resources) {
    if (resources.r3000.cp2.gc[31].read_bitfield(Bitfield::new(13, 6)) > 0) || (resources.r3000.cp2.gc[31].read_bitfield(Bitfield::new(23, 8)) > 0) {
        resources.r3000.cp2.gc[31].write_bitfield(Bitfield::new(31, 1), 1);
    }
}

pub fn handle_cp2_push_rgb(resources: &mut Resources) {
    let rgb1_value = resources.r3000.cp2.gd[21].read_u32();
    let rgb2_value = resources.r3000.cp2.gd[22].read_u32();
    resources.r3000.cp2.gd[20].write_u32(rgb1_value); // RGB0 = RGB1
    resources.r3000.cp2.gd[13].write_u32(rgb2_value); // RGB1 = RGB2
}
