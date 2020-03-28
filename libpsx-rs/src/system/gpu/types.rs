use std::collections::VecDeque;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::system::Resources;
use crate::system::gpu::crtc::Crtc;
use crate::system::gpu::register::{Gpu1810, Gpu1814};
use crate::system::gpu::data::*;
use std::time::Duration;
use log::warn;
use crate::types::register::b32_register::B32Register;
use crate::types::b8_memory_mapper::*;
use crate::types::fifo::Fifo;
use crate::types::fifo::debug::DebugState;

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

pub struct Gpu1810 {
    pub gp0: Fifo<u32>,
    pub read: Fifo<u32>, 
}

impl Gpu1810 {
    pub fn new() -> Gpu1810 {
        Gpu1810 {
            gp0: Fifo::new(64, Some(DebugState::new("GPU GP0", false, false))),
            read: Fifo::new(64, Some(DebugState::new("GPU READ", false, false))),
        }
    }
}

impl B8MemoryMap for Gpu1810 {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        assert!(offset == 0, "Invalid offset");
        
        Ok(self.read.read_one().unwrap_or_else(|_| {
            warn!("GPUREAD is empty - returning 0xFFFF_FFFF");
            0xFFFF_FFFF
        }))
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.gp0.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct Gpu1814 {
    pub gp1: Fifo<u32>, 
    pub stat: B32Register,
}

impl Gpu1814 {
    pub fn new() -> Gpu1814 {
        Gpu1814 {
            gp1: Fifo::new(64, Some(DebugState::new("GPU GP1", false, false))),
            stat: B32Register::new(),
        }
    }
}

impl B8MemoryMap for Gpu1814 {
    fn read_u32(&mut self, offset: u32) -> ReadResult<u32> {
        B8MemoryMap::read_u32(&mut self.stat, offset)
    }
    
    fn write_u32(&mut self, offset: u32, value: u32) -> WriteResult {
        assert!(offset == 0, "Invalid offset");
        self.gp1.write_one(value).map_err(|_| WriteError::Full)
    }
}

pub struct Gpu {
    pub gpu1810: Gpu1810,
    pub gpu1814: Gpu1814,
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

    pub crtc: Crtc,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            gpu1810: Gpu1810::new(),
            gpu1814: Gpu1814::new(),
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
            crtc: Crtc::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map(0x1F80_1810, 4, &mut resources.gpu.gpu1810 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map(0x1F80_1814, 4, &mut resources.gpu.gpu1814 as *mut dyn B8MemoryMap);
}
