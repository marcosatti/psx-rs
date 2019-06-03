use crate::State;
use crate::types::bitfield::Bitfield;
use crate::resources::gpu::*;
use crate::controllers::gpu::gp0;

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;
    let fifo = &mut resources.gpu.gpu1814.gp1;

    let command = fifo.peek_front();
    if let Some(value) = command {
        let cmd = GP_CMD.extract_from(value) as u8;

        match cmd {
            0x00 => {
                fifo.read_one().unwrap();
                command_00(state);
            },
            0x01 => {
                fifo.read_one().unwrap();
                command_01(state);
            },
            0x02 => {
                fifo.read_one().unwrap();
                command_02(state);
            },
            0x03 => command_03(state, fifo.read_one().unwrap()),
            0x04 => command_04(state, fifo.read_one().unwrap()),
            0x05 => command_05(state, fifo.read_one().unwrap()),
            0x06 => command_06(state, fifo.read_one().unwrap()),
            0x07 => command_07(state, fifo.read_one().unwrap()),
            0x08 => command_08(state, fifo.read_one().unwrap()),
            _ => unimplemented!("Unknown GP1 command: 0x{:0X}", value),
        }
    }
}

unsafe fn command_00(state: &State) {
    command_01(state);
    command_02(state);
    command_03(state, 1);
    command_04(state, 0);
    command_05(state, 0);
    command_06(state, 0);
    command_07(state, 0);
    command_08(state, 0);
    gp0::command_e1(state, 0);
    gp0::command_e2(state, 0);
    gp0::command_e3(state, 0);
    gp0::command_e4(state, 0);
    gp0::command_e5(state, 0);
    gp0::command_e6(state, 0);
}

unsafe fn command_01(state: &State) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1810.gp0.clear();
}

unsafe fn command_02(state: &State) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1814.stat.write_bitfield(STAT_IRQ_REQUEST, 0);
}

unsafe fn command_03(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1814.stat.write_bitfield(STAT_DISPLAY_ENABLE, Bitfield::new(0, 1).extract_from(command));
}

unsafe fn command_04(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.gpu1814.stat.write_bitfield(STAT_DMA_DIRECTION, Bitfield::new(0, 2).extract_from(command));
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
