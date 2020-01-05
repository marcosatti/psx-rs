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

pub fn checked_clamp<T: PartialOrd>(input: T, min: T, max: T) -> (T, bool) {
    if input < min {
        (min, true)
    } else if input > max {
        (max, true)
    } else {
        (input, false)
    }
}
