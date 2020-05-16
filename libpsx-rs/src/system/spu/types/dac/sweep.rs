#[derive(Copy, Clone, Debug)]
pub enum SweepMode {
    Linear,
    Exponential,
}

#[derive(Copy, Clone, Debug)]
pub enum SweepDirection {
    Increase,
    Decrease,
}

#[derive(Copy, Clone, Debug)]
pub enum SweepPhase {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Debug)]
pub struct SweepParams {
    pub step: usize,
    pub shift: usize,
    pub phase: SweepPhase,
    pub direction: SweepDirection,
    pub mode: SweepMode,
}
