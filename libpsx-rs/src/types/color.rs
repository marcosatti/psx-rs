use crate::types::{
    array::AsFlattened,
    bitfield::Bitfield,
};
use smallvec::SmallVec;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    pub(crate) const fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    pub(crate) fn as_flat(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub(crate) fn to_normalized(&self) -> NormalizedColor {
        NormalizedColor::new(self.r as f32 / std::u8::MAX as f32, self.g as f32 / std::u8::MAX as f32, self.b as f32 / std::u8::MAX as f32)
    }
}

impl AsFlattened for [Color] {
    type Output = u8;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]> {
        let mut buffer = SmallVec::new();

        for item in self.iter() {
            for component in item.as_flat().iter() {
                buffer.push(*component);
            }
        }

        buffer
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct NormalizedColor {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl NormalizedColor {
    pub(crate) fn new(r: f32, g: f32, b: f32) -> NormalizedColor {
        NormalizedColor {
            r,
            g,
            b,
        }
    }

    pub(crate) fn as_flat(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl AsFlattened for [NormalizedColor] {
    type Output = f32;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]> {
        let mut buffer = SmallVec::new();

        for item in self.iter() {
            for component in item.as_flat().iter() {
                buffer.push(*component);
            }
        }

        buffer
    }
}

/// 5551 RGBA Packed Color.
#[derive(Copy, Clone, Debug)]
pub(crate) struct PackedColor {
    pub(crate) color: u16,
}

impl PackedColor {
    pub(crate) fn new(color: u16) -> PackedColor {
        PackedColor {
            color,
        }
    }

    pub(crate) fn from_x2(packed: u32) -> [PackedColor; 2] {
        [PackedColor::new(Bitfield::new(0, 16).extract_from(packed) as u16), PackedColor::new(Bitfield::new(16, 16).extract_from(packed) as u16)]
    }
}

impl AsFlattened for [PackedColor] {
    type Output = u16;

    fn as_flattened(&self) -> SmallVec<[Self::Output; 16]> {
        let mut buffer = SmallVec::new();

        for item in self.iter() {
            buffer.push(item.color);
        }

        buffer
    }
}
