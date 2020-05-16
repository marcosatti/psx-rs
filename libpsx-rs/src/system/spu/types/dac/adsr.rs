#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdsrPhase {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdsrMode {
    Linear,
    Exponential,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdsrDirection {
    Increase,
    Decrease,
}

#[derive(Copy, Clone, Debug)]
pub struct AdsrPhaseParams {
    /// Note: this is the raw step value (0 ..= 3), not the final step value (-8 ..= +7).
    pub step: usize,
    pub shift: usize,
    pub direction: AdsrDirection,
    pub mode: AdsrMode,
}

#[derive(Copy, Clone, Debug)]
pub struct AdsrState {
    /// ADSR phase (attack, decay, sustain, release).
    pub phase: AdsrPhase,
    /// ADSR volume.
    pub current_volume: i16,
    /// ADSR next volume.
    pub next_volume: i16,
    /// ADSR wait cycles.
    /// The number of cycles to wait before applying the next volume.
    pub wait_cycles: usize,
}

impl AdsrState {
    pub fn new() -> AdsrState {
        AdsrState {
            phase: AdsrPhase::Attack,
            current_volume: 0,
            next_volume: 0,
            wait_cycles: 0,
        }
    }
}
