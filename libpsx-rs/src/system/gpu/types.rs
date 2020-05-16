use crate::{
    system::gpu::crtc::types::Crtc,
    types::{
        fifo::{
            debug::DebugState,
            Fifo,
        },
        memory::*,
    },
};
use parking_lot::Mutex;
use std::collections::VecDeque;

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

pub struct ControllerState {
    pub textured_rect_x_flip: bool,
    pub textured_rect_y_flip: bool,
    pub display_area_start_x: usize,
    pub display_area_start_y: usize,
    pub horizontal_display_range_x1: usize,
    pub horizontal_display_range_x2: usize,
    pub vertical_display_range_y1: usize,
    pub vertical_display_range_y2: usize,
    pub texture_window_mask_x: usize,
    pub texture_window_mask_y: usize,
    pub texture_window_offset_x: isize,
    pub texture_window_offset_y: isize,
    pub drawing_area_x1: usize,
    pub drawing_area_y1: usize,
    pub drawing_area_x2: usize,
    pub drawing_area_y2: usize,
    pub drawing_offset_x: isize,
    pub drawing_offset_y: isize,
    pub texpage_base_x: isize,
    pub texpage_base_y: isize,
    pub clut_mode: ClutMode,
    pub transparency_mode: TransparencyMode,
    pub gp0_command_buffer: Vec<u32>,
    pub gp0_command_required_length: Option<usize>,
    pub gp0_read_buffer: VecDeque<u32>,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        ControllerState {
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

pub struct State {
    pub crtc: Crtc,
    pub gp0: Fifo<u32>,
    pub read: Fifo<u32>,
    pub gp1: Fifo<u32>,
    pub stat: B32LevelRegister,
    pub controller_state: Mutex<ControllerState>,
}

impl State {
    pub fn new() -> State {
        State {
            crtc: Crtc::new(),
            gp0: Fifo::new(64, Some(DebugState::new("GPU GP0", false, false))),
            read: Fifo::new(64, Some(DebugState::new("GPU READ", false, false))),
            gp1: Fifo::new(64, Some(DebugState::new("GPU GP1", false, false))),
            stat: B32LevelRegister::new(),
            controller_state: Mutex::new(ControllerState::new()),
        }
    }
}
