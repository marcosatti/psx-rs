#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn normalize(&self) -> (f32, f32, f32, f32) { 
        let divisor = std::u8::MAX as f32;
        (self.r as f32 / divisor, self.g as f32 / divisor, self.b as f32 / divisor, self.a as f32 / divisor)
    }
}
