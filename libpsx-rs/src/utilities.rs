pub mod mips1;
pub mod numeric;
pub mod packed;

pub fn bool_to_flag(value: bool) -> u32 { 
    if value {
        1 
    } else { 
        0 
    } 
}
