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
        const MAX: u16 = std::u8::MAX as u16;
        const DIVISOR: u16 = ((1 << 5) - 1) as u16;
        let r = ((Bitfield::new(0, 5).extract_from(packed) * MAX) / DIVISOR) as u8;
        let g = ((Bitfield::new(5, 5).extract_from(packed) * MAX) / DIVISOR) as u8;
        let b = ((Bitfield::new(10, 5).extract_from(packed) * MAX) / DIVISOR) as u8;
        let a = (Bitfield::new(15, 1).extract_from(packed) * MAX) as u8;
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
