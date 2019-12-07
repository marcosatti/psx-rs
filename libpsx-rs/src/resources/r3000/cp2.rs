use crate::resources::Resources;
use crate::types::register::b32_register::B32Register;

pub struct Cp2 {
    /// Data registers.
    pub gd: [B32Register; 32],
    /// Control registers.
    pub gc: [B32Register; 32],
}

impl Cp2 {
    pub fn new() -> Cp2 {
        Cp2 {
            gd: [B32Register::new(); 32],
            gc: [B32Register::new(); 32],
        }
    }
}

pub fn initialize(_resources: &mut Resources) {
}
