#[derive(Copy, Clone, Debug)]
pub enum TransparencyMode {
    Average,
    Additive,
    Difference,
    Quarter,
}

#[derive(Copy, Clone, Debug)]
pub enum ClutMode {
    Bits4,
    Bits8,
    Bits15,
    Reserved,
}
