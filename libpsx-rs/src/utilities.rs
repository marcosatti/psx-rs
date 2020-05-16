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

pub fn binary_to_ascii_escaped(data: &[u8]) -> String {
    let mut binary_ascii_str = String::new();

    for &byte in data.iter() {
        let part: Vec<u8> = std::ascii::escape_default(byte).collect();
        binary_ascii_str.push_str(std::str::from_utf8(&part).unwrap());
    }

    binary_ascii_str
}
