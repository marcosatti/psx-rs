pub(crate) mod mips1;
pub(crate) mod numeric;
pub(crate) mod packed;
pub(crate) mod primitive;

pub(crate) fn bool_to_flag(value: bool) -> u32 {
    if value {
        1
    } else {
        0
    }
}

pub(crate) fn checked_clamp<T: PartialOrd>(input: T, min: T, max: T) -> (T, bool) {
    if input < min {
        (min, true)
    } else if input > max {
        (max, true)
    } else {
        (input, false)
    }
}

pub(crate) fn binary_to_ascii_escaped(data: &[u8]) -> String {
    let mut binary_ascii_str = String::new();

    for &byte in data.iter() {
        let part: Vec<u8> = std::ascii::escape_default(byte).collect();
        binary_ascii_str.push_str(std::str::from_utf8(&part).unwrap());
    }

    binary_ascii_str
}
