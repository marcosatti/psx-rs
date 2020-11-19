use crate::types::bitfield::Bitfield;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
    pub(crate) a: f32,
}

impl Color {
    pub(crate) fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub(crate) fn from_8888(r: u8, g: u8, b: u8, a: u8) -> Color {
        const DIVISOR: f32 = std::u8::MAX as f32;
        let r = r as f32 / DIVISOR;
        let g = g as f32 / DIVISOR;
        let b = b as f32 / DIVISOR;
        let a = a as f32 / DIVISOR;
        Color::new(r, g, b, a)
    }

    pub(crate) fn from_packed_888(packed: u32, a: u8) -> Color {
        let r = Bitfield::new(0, 8).extract_from(packed) as u8;
        let g = Bitfield::new(8, 8).extract_from(packed) as u8;
        let b = Bitfield::new(16, 8).extract_from(packed) as u8;
        Color::from_8888(r, g, b, a)
    }

    pub(crate) fn from_packed_5551(packed: u16) -> Color {
        const DIVISOR: f32 = ((1u16 << 5) - 1) as f32;
        let r = Bitfield::new(0, 5).extract_from(packed) as f32 / DIVISOR;
        let g = Bitfield::new(5, 5).extract_from(packed) as f32 / DIVISOR;
        let b = Bitfield::new(10, 5).extract_from(packed) as f32 / DIVISOR;
        let a = Bitfield::new(15, 1).extract_from(packed) as f32;
        Color::new(r, g, b, a)
    }

    pub(crate) fn from_packed_5551_x2(packed: u32) -> [Color; 2] {
        [Color::from_packed_5551(Bitfield::new(0, 16).extract_from(packed) as u16), Color::from_packed_5551(Bitfield::new(16, 16).extract_from(packed) as u16)]
    }

    pub(crate) fn as_flat(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}
