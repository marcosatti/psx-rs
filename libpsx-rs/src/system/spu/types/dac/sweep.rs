#[derive(Copy, Clone, Debug)]
pub(crate) enum SweepMode {
    Linear,
    Exponential,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum SweepDirection {
    Increase,
    Decrease,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum SweepPhase {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct SweepParams {
    pub(crate) step: usize,
    pub(crate) shift: usize,
    pub(crate) phase: SweepPhase,
    pub(crate) direction: SweepDirection,
    pub(crate) mode: SweepMode,
}
