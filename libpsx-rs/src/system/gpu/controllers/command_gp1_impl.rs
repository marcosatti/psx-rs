use crate::backends::video::VideoBackend;
use crate::system::types::State;
use crate::types::bitfield::Bitfield;
use crate::system::gpu::constants::*;
use crate::system::gpu::controllers::command_gp0_impl;

pub fn command_00(state: &mut State, video_backend: &VideoBackend, _command: u32) {
    command_01(state, video_backend, 0);
    command_02(state, video_backend, 0);
    command_03(state, video_backend, 1);
    command_04(state, video_backend, 0);
    command_05(state, video_backend, 0);
    command_06(state, video_backend, 0);
    command_07(state, video_backend, 0);
    command_08(state, video_backend, 0);
    command_gp0_impl::command_e1_handler(state, video_backend, &[0]);
    command_gp0_impl::command_e2_handler(state, video_backend, &[0]);
    command_gp0_impl::command_e3_handler(state, video_backend, &[0]);
    command_gp0_impl::command_e4_handler(state, video_backend, &[0]);
    command_gp0_impl::command_e5_handler(state, video_backend, &[0]);
    command_gp0_impl::command_e6_handler(state, video_backend, &[0]);
}

pub fn command_01(state: &mut State, _video_backend: &VideoBackend, _command: u32) {
    state.gpu.gpu1810.gp0.clear();
}

pub fn command_02(state: &mut State, _video_backend: &VideoBackend, _command: u32) {
    state.gpu.gpu1814.stat.write_bitfield(STAT_IRQ_REQUEST, 0);
}

pub fn command_03(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    state.gpu.gpu1814.stat.write_bitfield(STAT_DISPLAY_ENABLE, Bitfield::new(0, 1).extract_from(command));
}

pub fn command_04(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    let dma_direction = Bitfield::new(0, 2).extract_from(command);
    
    match dma_direction {
        0 => {
            //debug!("DMA direction set to 0 (off)");
        }
        1 => {
            //debug!("DMA direction set to 1 (FIFO) - what does this mean???");
        },
        2 => {
            //debug!("DMA direction set to 2 (CPUtoGP0)");
        },
        3 => {
            //debug!("DMA direction set to 3 (GPUREADtoCPU)");
        },
        _ => unreachable!(),
    }

    state.gpu.gpu1814.stat.write_bitfield(STAT_DMA_DIRECTION, dma_direction);
}

pub fn command_05(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    state.gpu.display_area_start_x = Bitfield::new(0, 10).extract_from(command) as usize;
    state.gpu.display_area_start_y = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub fn command_06(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    state.gpu.horizontal_display_range_x1 = Bitfield::new(0, 12).extract_from(command) as usize;
    state.gpu.horizontal_display_range_x2 = Bitfield::new(12, 12).extract_from(command) as usize;
}

pub fn command_07(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    state.gpu.vertical_display_range_y1 = Bitfield::new(0, 10).extract_from(command) as usize;
    state.gpu.vertical_display_range_y2 = Bitfield::new(10, 10).extract_from(command) as usize;
}

pub fn command_08(state: &mut State, _video_backend: &VideoBackend, command: u32) {
    let stat = &mut state.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_HORIZONTAL_RES_1, Bitfield::new(0, 2).extract_from(command));
    stat.write_bitfield(STAT_VERTICAL_RES, Bitfield::new(2, 1).extract_from(command));
    stat.write_bitfield(STAT_VIDEO_MODE, Bitfield::new(3, 1).extract_from(command));
    stat.write_bitfield(STAT_DISPLAY_COLOR_DEPTH, Bitfield::new(4, 1).extract_from(command));
    stat.write_bitfield(STAT_INTERLACE_VERTICAL, Bitfield::new(5, 1).extract_from(command));
    stat.write_bitfield(STAT_HORIZONTAL_RES_2, Bitfield::new(6, 1).extract_from(command));
    stat.write_bitfield(STAT_REVERSE, Bitfield::new(7, 1).extract_from(command));
}
