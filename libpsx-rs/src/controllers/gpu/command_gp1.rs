use crate::State;
use crate::types::bitfield::Bitfield;
use crate::resources::gpu::*;
use crate::controllers::gpu::command_gp0;

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;
    let fifo = &mut resources.gpu.gpu1814.gp1;

    // Commands (GP1) are always of length 1.

    let command = match fifo.read_one() {
        Ok(v) => v,
        Err(_) => return,
    };

    let command_index = GP_CMD.extract_from(command) as u8;

    let command_fn = match command_index {
        0x00 => command_00,
        0x01 => command_01,
        0x02 => command_02,
        0x03 => command_03,
        0x04 => command_04,
        0x05 => command_05,
        0x06 => command_06,
        0x07 => command_07,
        0x08 => command_08,
        _ => unimplemented!("Unknown GP1 command: 0x{:0X}", command_index),
    };

    command_fn(state, command);
}

unsafe fn command_00(state: &State, _command: u32) {
    command_01(state, 0);
    command_02(state, 0);
    command_03(state, 1);
    command_04(state, 0);
    command_05(state, 0);
    command_06(state, 0);
    command_07(state, 0);
    command_08(state, 0);
    (command_gp0::COMMAND_E1.handler_fn)(state, &[0]);
    (command_gp0::COMMAND_E2.handler_fn)(state, &[0]);
    (command_gp0::COMMAND_E3.handler_fn)(state, &[0]);
    (command_gp0::COMMAND_E4.handler_fn)(state, &[0]);
    (command_gp0::COMMAND_E5.handler_fn)(state, &[0]);
    (command_gp0::COMMAND_E6.handler_fn)(state, &[0]);
}

unsafe fn command_01(state: &State, _command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1810.gp0.clear();
}

unsafe fn command_02(state: &State, _command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1814.stat.write_bitfield(STAT_IRQ_REQUEST, 0);
}

unsafe fn command_03(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1814.stat.write_bitfield(STAT_DISPLAY_ENABLE, Bitfield::new(0, 1).extract_from(command));
}

unsafe fn command_04(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let dma_direction = Bitfield::new(0, 2).extract_from(command);
    if dma_direction == 3 {
        unimplemented!("DMA direction set to 3 (GPUREAD to CPU)");
    }
    resources.gpu.gpu1814.stat.write_bitfield(STAT_DMA_DIRECTION, dma_direction);
}

unsafe fn command_05(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.display_area_start_x = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.display_area_start_y = Bitfield::new(10, 9).extract_from(command) as usize;
}

unsafe fn command_06(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.horizontal_display_range_x1 = Bitfield::new(0, 12).extract_from(command) as usize;
    resources.gpu.horizontal_display_range_x2 = Bitfield::new(12, 12).extract_from(command) as usize;
}

unsafe fn command_07(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.vertical_display_range_y1 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.vertical_display_range_y2 = Bitfield::new(10, 10).extract_from(command) as usize;
}

unsafe fn command_08(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_HORIZONTAL_RES_1, Bitfield::new(0, 2).extract_from(command));
    stat.write_bitfield(STAT_VERTICAL_RES, Bitfield::new(2, 1).extract_from(command));
    stat.write_bitfield(STAT_VIDEO_MODE, Bitfield::new(3, 1).extract_from(command));
    stat.write_bitfield(STAT_DISPLAY_COLOR_DEPTH, Bitfield::new(4, 1).extract_from(command));
    stat.write_bitfield(STAT_INTERLACE_VERTICAL, Bitfield::new(5, 1).extract_from(command));
    stat.write_bitfield(STAT_HORIZONTAL_RES_2, Bitfield::new(6, 1).extract_from(command));
    stat.write_bitfield(STAT_REVERSE, Bitfield::new(7, 1).extract_from(command));
}
