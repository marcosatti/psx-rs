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

    pub(crate) fn normalize(&self) -> (f32, f32, f32, f32) {
        let divisor = std::u8::MAX as f32;
        (self.r as f32 / divisor, self.g as f32 / divisor, self.b as f32 / divisor, self.a as f32 / divisor)
    }
}
