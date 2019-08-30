use log::trace;
use crate::State;

static REGISTER_NAMES: [&str; 32] = [
    "zero",
    "at",
    "v0",
    "v1",
    "a0",
    "a1",
    "a2",
    "a3",
    "t0",
    "t1",
    "t2",
    "t3",
    "t4",
    "t5",
    "t6",
    "t7",
    "s0",
    "s1",
    "s2",
    "s3",
    "s4",
    "s5",
    "s6",
    "s7",
    "t8",
    "t9",
    "k0",
    "k1",
    "gp",
    "sp",
    "fp",
    "ra",
];

pub unsafe fn trace_registers(state: &State) {
    let resources = &mut *state.resources;

    let mut string = String::new();
    string.push_str("Register dump:\n");
    for (index, (ref register, name)) in resources.r3000.gpr.iter().zip(REGISTER_NAMES.iter()).enumerate() {
        string.push_str(&format!("{:>4} = 0x{:08X}, ", name, register.read_u32()));
        
        if (index + 1) % 4 == 0 {
            string.push_str("\n");
        }
    }

    trace!("{}", &string);
}
