use crate::{
    system::gpu::crtc::types::Crtc,
    types::{
        fifo::Fifo,
        memory::*,
    },
};
use crate::types::exclusive_state::ExclusiveState;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum TransparencyMode {
    Average,
    Additive,
    Difference,
    Quarter,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub(crate) enum ClutMode {
    Bits4,
    Bits8,
    Bits15,
    Reserved,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    /// Synchronization state.
    pub(crate) clock: f64,
    pub(crate) gp1_command: Option<u32>,
    pub(crate) textured_rect_x_flip: bool,
    pub(crate) textured_rect_y_flip: bool,
    pub(crate) display_area_start_x: usize,
    pub(crate) display_area_start_y: usize,
    pub(crate) horizontal_display_range_x1: usize,
    pub(crate) horizontal_display_range_x2: usize,
    pub(crate) vertical_display_range_y1: usize,
    pub(crate) vertical_display_range_y2: usize,
    pub(crate) texture_window_mask_x: usize,
    pub(crate) texture_window_mask_y: usize,
    pub(crate) texture_window_offset_x: isize,
    pub(crate) texture_window_offset_y: isize,
    pub(crate) drawing_area_x1: usize,
    pub(crate) drawing_area_y1: usize,
    pub(crate) drawing_area_x2: usize,
    pub(crate) drawing_area_y2: usize,
    pub(crate) drawing_offset_x: isize,
    pub(crate) drawing_offset_y: isize,
    pub(crate) texpage_base_x: isize,
    pub(crate) texpage_base_y: isize,
    pub(crate) clut_mode: ClutMode,
    pub(crate) transparency_mode: TransparencyMode,
    pub(crate) gp0_command_buffer: Vec<u32>,
    pub(crate) gp0_command_required_length: Option<usize>,
    pub(crate) gp0_read_buffer: VecDeque<u32>,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
            gp1_command: None,
            textured_rect_x_flip: false,
            textured_rect_y_flip: false,
            display_area_start_x: 0,
            display_area_start_y: 0,
            horizontal_display_range_x1: 0,
            horizontal_display_range_x2: 0,
            vertical_display_range_y1: 0,
            vertical_display_range_y2: 0,
            texture_window_mask_x: 0,
            texture_window_mask_y: 0,
            texture_window_offset_x: 0,
            texture_window_offset_y: 0,
            drawing_area_x1: 0,
            drawing_area_y1: 0,
            drawing_area_x2: 0,
            drawing_area_y2: 0,
            drawing_offset_x: 0,
            drawing_offset_y: 0,
            texpage_base_x: 0,
            texpage_base_y: 0,
            clut_mode: ClutMode::Bits4,
            transparency_mode: TransparencyMode::Average,
            gp0_command_buffer: Vec::new(),
            gp0_command_required_length: None,
            gp0_read_buffer: VecDeque::new(),
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) crtc: Crtc,
    pub(crate) gp0: Fifo<u32>,
    pub(crate) read: Fifo<u32>,
    pub(crate) gp1: B32EdgeRegister,
    pub(crate) stat: B32LevelRegister,
    pub(crate) controller_state: ExclusiveState<ControllerState>,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            crtc: Crtc::new(),
            gp0: Fifo::new(2048),
            read: Fifo::new(2048),
            gp1: B32EdgeRegister::new(),
            stat: B32LevelRegister::new(),
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
