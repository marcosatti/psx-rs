use crate::types::bitfield::Bitfield;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Color {
    pub(crate) fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub(crate) fn from_packed_5551(packed: u16) -> Color {
        let r = ((Bitfield::new(0, 5).extract_from(packed) * 255) / 31) as u8;
        let g = ((Bitfield::new(5, 5).extract_from(packed) * 255) / 31) as u8;
        let b = ((Bitfield::new(10, 5).extract_from(packed) * 255) / 31) as u8;
        let a = if Bitfield::new(15, 1).extract_from(packed) != 0 {
            std::u8::MAX
        } else {
            0
        };
        Color::new(r, g, b, a)
    }

    pub(crate) fn from_packed_5551_x2(packed: u32) -> [Color; 2] {
        [Color::from_packed_5551(Bitfield::new(0, 16).extract_from(packed) as u16), Color::from_packed_5551(Bitfield::new(16, 16).extract_from(packed) as u16)]
    }

    pub(crate) fn normalize(&self) -> (f32, f32, f32, f32) {
        let divisor = std::u8::MAX as f32;
        (self.r as f32 / divisor, self.g as f32 / divisor, self.b as f32 / divisor, self.a as f32 / divisor)
    }
}
