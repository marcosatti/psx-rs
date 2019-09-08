use crate::backends::video::VideoBackend;
use crate::resources::Resources;
use crate::resources::gpu::*;
use crate::controllers::gpu::command_gp1_impl;

pub fn handle_command<'a>(resources: &mut Resources, video_backend: &VideoBackend<'a>) {
    let fifo = &mut resources.gpu.gpu1814.gp1;

    // Commands (GP1) are always of length 1.

    let command = match fifo.read_one() {
        Ok(v) => v,
        Err(_) => return,
    };

    let command_index = GP_CMD.extract_from(command) as u8;

    let command_fn = match command_index {
        0x00 => command_gp1_impl::command_00,
        0x01 => command_gp1_impl::command_01,
        0x02 => command_gp1_impl::command_02,
        0x03 => command_gp1_impl::command_03,
        0x04 => command_gp1_impl::command_04,
        0x05 => command_gp1_impl::command_05,
        0x06 => command_gp1_impl::command_06,
        0x07 => command_gp1_impl::command_07,
        0x08 => command_gp1_impl::command_08,
        _ => unimplemented!("Unknown GP1 command: 0x{:0X}", command_index),
    };

    command_fn(resources, video_backend, command);
}
