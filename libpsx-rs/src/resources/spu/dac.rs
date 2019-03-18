use std::time::Duration;
use crate::resources::spu::voice::*;

pub struct Dac {
    pub current_duration: Duration,
    pub voice0_state: PlayState,
    pub voice1_state: PlayState,
    pub voice2_state: PlayState,
    pub voice3_state: PlayState,
    pub voice4_state: PlayState,
    pub voice5_state: PlayState,
    pub voice6_state: PlayState,
    pub voice7_state: PlayState,
    pub voice8_state: PlayState,
    pub voice9_state: PlayState,
    pub voice10_state: PlayState,
    pub voice11_state: PlayState,
    pub voice12_state: PlayState,
    pub voice13_state: PlayState,
    pub voice14_state: PlayState,
    pub voice15_state: PlayState,
    pub voice16_state: PlayState,
    pub voice17_state: PlayState,
    pub voice18_state: PlayState,
    pub voice19_state: PlayState,
    pub voice20_state: PlayState,
    pub voice21_state: PlayState,
    pub voice22_state: PlayState,
    pub voice23_state: PlayState,
}

impl Dac {
    pub fn new() -> Dac {
        Dac {
            current_duration: Duration::from_secs(0),
            voice0_state: PlayState::new(),
            voice1_state: PlayState::new(),
            voice2_state: PlayState::new(),
            voice3_state: PlayState::new(),
            voice4_state: PlayState::new(),
            voice5_state: PlayState::new(),
            voice6_state: PlayState::new(),
            voice7_state: PlayState::new(),
            voice8_state: PlayState::new(),
            voice9_state: PlayState::new(),
            voice10_state: PlayState::new(),
            voice11_state: PlayState::new(),
            voice12_state: PlayState::new(),
            voice13_state: PlayState::new(),
            voice14_state: PlayState::new(),
            voice15_state: PlayState::new(),
            voice16_state: PlayState::new(),
            voice17_state: PlayState::new(),
            voice18_state: PlayState::new(),
            voice19_state: PlayState::new(),
            voice20_state: PlayState::new(),
            voice21_state: PlayState::new(),
            voice22_state: PlayState::new(),
            voice23_state: PlayState::new(),
        }
    }
}
