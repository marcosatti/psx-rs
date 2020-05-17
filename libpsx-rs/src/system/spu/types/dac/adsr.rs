#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum AdsrPhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum AdsrMode {
    Linear,
    Exponential,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum AdsrDirection {
    Increase,
    Decrease,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct AdsrPhaseParams {
    /// Note: this is the raw step value (0 ..= 3), not the final step value (-8 ..= +7).
    pub(crate) step: usize,
    pub(crate) shift: usize,
    pub(crate) direction: AdsrDirection,
    pub(crate) mode: AdsrMode,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct AdsrState {
    /// ADSR phase (attack, decay, sustain, release).
    pub(crate) phase: AdsrPhase,
    /// ADSR volume.
    pub(crate) current_volume: i16,
    /// ADSR next volume.
    pub(crate) next_volume: i16,
    /// ADSR wait cycles.
    /// The number of cycles to wait before applying the next volume.
    pub(crate) wait_cycles: usize,
}

impl AdsrState {
    pub(crate) fn new() -> AdsrState {
        AdsrState {
            phase: AdsrPhase::Attack,
            current_volume: 0,
            next_volume: 0,
            wait_cycles: 0,
        }
    }
}
