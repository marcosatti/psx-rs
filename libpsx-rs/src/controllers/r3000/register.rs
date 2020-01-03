use crate::resources::Resources;

pub fn handle_registers(resources: &mut Resources) {
    handle_zero(resources);
    handle_cp2_sxyp(resources);
}

fn handle_zero(resources: &mut Resources) {
    resources.r3000.gpr[0].write_u32(0);
}

fn handle_cp2_sxyp(resources: &mut Resources) {
    let value = resources.r3000.cp2.gc[14].read_u32();
    resources.r3000.cp2.gc[15].write_u32(value);
}
