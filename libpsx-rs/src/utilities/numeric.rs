use fixed::{
    traits::{FromFixed, ToFixed},
    types::extra::{LeEqU16, LeEqU32},
    FixedI16, FixedI32, FixedU16, FixedU32,
};

pub trait FromFixedBits {
    fn from_fixed_bits_u16<U: LeEqU16>(value: u16) -> Self;
    fn from_fixed_bits_i16<U: LeEqU16>(value: i16) -> Self;
    fn from_fixed_bits_u32<U: LeEqU32>(value: u32) -> Self;
    fn from_fixed_bits_i32<U: LeEqU32>(value: i32) -> Self;
}

impl FromFixedBits for f64 {
    fn from_fixed_bits_u16<U: LeEqU16>(value: u16) -> Self {
        f64::from_fixed(FixedU16::<U>::from_bits(value))
    }

    fn from_fixed_bits_i16<U: LeEqU16>(value: i16) -> Self {
        f64::from_fixed(FixedI16::<U>::from_bits(value))
    }

    fn from_fixed_bits_u32<U: LeEqU32>(value: u32) -> Self {
        f64::from_fixed(FixedU32::<U>::from_bits(value))
    }

    fn from_fixed_bits_i32<U: LeEqU32>(value: i32) -> Self {
        f64::from_fixed(FixedI32::<U>::from_bits(value))
    }
}

pub trait ToFixedBits {
    fn to_fixed_bits_u16<U: LeEqU16>(self, saturate: bool) -> u16;
    fn to_fixed_bits_i16<U: LeEqU16>(self, saturate: bool) -> i16;
    fn to_fixed_bits_u32<U: LeEqU32>(self, saturate: bool) -> u32;
    fn to_fixed_bits_i32<U: LeEqU32>(self, saturate: bool) -> i32;
}

impl ToFixedBits for f64 {
    fn to_fixed_bits_u16<U: LeEqU16>(self, saturate: bool) -> u16 {
        if saturate {
            f64::saturating_to_fixed::<FixedU16<U>>(self).to_bits()
        } else {
            f64::wrapping_to_fixed::<FixedU16<U>>(self).to_bits()
        }
    }

    fn to_fixed_bits_i16<U: LeEqU16>(self, saturate: bool) -> i16 {
        if saturate {
            f64::saturating_to_fixed::<FixedI16<U>>(self).to_bits()
        } else {
            f64::wrapping_to_fixed::<FixedI16<U>>(self).to_bits()
        }
    }

    fn to_fixed_bits_u32<U: LeEqU32>(self, saturate: bool) -> u32 {
        if saturate {
            f64::saturating_to_fixed::<FixedU32<U>>(self).to_bits()
        } else {
            f64::wrapping_to_fixed::<FixedU32<U>>(self).to_bits()
        }
    }

    fn to_fixed_bits_i32<U: LeEqU32>(self, saturate: bool) -> i32 {
        if saturate {
            f64::saturating_to_fixed::<FixedI32<U>>(self).to_bits()
        } else {
            f64::wrapping_to_fixed::<FixedI32<U>>(self).to_bits()
        }
    }
}
