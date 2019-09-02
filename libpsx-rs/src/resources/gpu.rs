pub mod crtc;
pub mod register;

use std::collections::VecDeque;
use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::types::bitfield::Bitfield;
use crate::resources::Resources;
use crate::resources::gpu::crtc::Crtc;
use crate::resources::gpu::register::{Gpu1810, Gpu1814};

pub const GP_CMD: Bitfield = Bitfield::new(24, 8);

pub const STAT_TEXPAGEX: Bitfield = Bitfield::new(0, 4);
pub const STAT_TEXPAGEY: Bitfield = Bitfield::new(4, 1);
pub const STAT_TRANSPARENCY: Bitfield = Bitfield::new(5, 2);
pub const STAT_TEXPAGE_COLORS: Bitfield = Bitfield::new(7, 2);
pub const STAT_DITHER: Bitfield = Bitfield::new(9, 1);
pub const STAT_DRAW_DISPLAY: Bitfield = Bitfield::new(10, 1);
pub const STAT_DRAW_MASK: Bitfield = Bitfield::new(11, 1);
pub const STAT_DRAW_PIXELS: Bitfield = Bitfield::new(12, 1);
pub const _STAT_INTERLACE_FIELD: Bitfield = Bitfield::new(13, 1);
pub const STAT_REVERSE: Bitfield = Bitfield::new(14, 1);
pub const STAT_TEXTURE_DISABLE: Bitfield = Bitfield::new(15, 1);
pub const STAT_HORIZONTAL_RES_2: Bitfield = Bitfield::new(16, 1);
pub const STAT_HORIZONTAL_RES_1: Bitfield = Bitfield::new(17, 2);
pub const STAT_VERTICAL_RES: Bitfield = Bitfield::new(19, 1);
pub const STAT_VIDEO_MODE: Bitfield = Bitfield::new(20, 1);
pub const STAT_DISPLAY_COLOR_DEPTH: Bitfield = Bitfield::new(21, 1);
pub const STAT_INTERLACE_VERTICAL: Bitfield = Bitfield::new(22, 1);
pub const STAT_DISPLAY_ENABLE: Bitfield = Bitfield::new(23, 1);
pub const STAT_IRQ_REQUEST: Bitfield = Bitfield::new(24, 1);
pub const STAT_DMA_REQUEST: Bitfield = Bitfield::new(25, 1);
pub const STAT_RECV_CMD: Bitfield = Bitfield::new(26, 1);
pub const STAT_SEND_VRAM: Bitfield = Bitfield::new(27, 1);
pub const STAT_RECV_DMA: Bitfield = Bitfield::new(28, 1);
pub const STAT_DMA_DIRECTION: Bitfield = Bitfield::new(29, 2);
pub const STAT_DRAWING_ODD: Bitfield = Bitfield::new(31, 1);

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
    pub texture_window_offset_x: usize,
    pub texture_window_offset_y: usize,
    pub drawing_area_x1: usize,
    pub drawing_area_y1: usize,
    pub drawing_area_x2: usize,
    pub drawing_area_y2: usize,
    pub drawing_offset_x: usize,
    pub drawing_offset_y: usize,
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
            gp0_command_buffer: Vec::new(),
            gp0_command_required_length: None,
            gp0_read_buffer: VecDeque::new(),
            crtc: Crtc::new(),
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1810, 4, &mut resources.gpu.gpu1810 as *mut dyn B8MemoryMap);
    resources.r3000.memory_mapper.map::<u32>(0x1F80_1814, 4, &mut resources.gpu.gpu1814 as *mut dyn B8MemoryMap);
}
