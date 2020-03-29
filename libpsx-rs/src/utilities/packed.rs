use crate::utilities::numeric::*;
use fixed::types::extra::LeEqU16;

pub fn split_32_fixedi16_f64<U: LeEqU16>(value: u32) -> (f64, f64) {
    (f64::from_fixed_bits_i16::<U>(value as u16 as i16), f64::from_fixed_bits_i16::<U>((value >> 16) as u16 as i16))
}

pub fn split_32_i16(value: u32) -> (i16, i16) {
    (value as u16 as i16, (value >> 16) as u16 as i16)
}

pub fn split_32_i16_f64(value: u32) -> (f64, f64) {
    let (value1, value2) = split_32_i16(value);
    (value1 as f64, value2 as f64)
}
