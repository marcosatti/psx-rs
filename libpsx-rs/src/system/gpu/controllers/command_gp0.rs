use crate::system::Resources;
use crate::backends::video::VideoBackend;
use crate::system::gpu::*;
use crate::controllers::gpu::command_gp0_impl;
use crate::controllers::gpu::debug;

/// Determines the amount of words needed to process the command.
type LengthFn = fn(&[u32]) -> Option<usize>;

/// The handler logic for the command.
type HandlerFn = fn(&mut Resources, video_backend: &VideoBackend, &[u32]);

pub fn handle_command(resources: &mut Resources, video_backend: &VideoBackend) {
    // Update the command buffer with any new incoming data.
    {
        let fifo = &mut resources.gpu.gpu1810.gp0;
        let command_buffer = &mut resources.gpu.gp0_command_buffer;

        loop {
            match fifo.read_one() {
                Ok(v) => command_buffer.push(v),
                Err(_) => break,
            }
        }

        // We cannot do anything yet.
        if command_buffer.is_empty() {
            return;
        }
    }

    // Get the associated command handler.
    let command_handler = {
        let command_buffer = &mut resources.gpu.gp0_command_buffer;
        let command = command_buffer[0];
        let command_index = GP_CMD.extract_from(command) as u8;
        get_command_handler(command_index)
    };

    // Try and get the required data length.
    let required_length_value = {
        let command_buffer = &mut resources.gpu.gp0_command_buffer;
        let required_length = &mut resources.gpu.gp0_command_required_length;

        if required_length.is_none() {
            match (command_handler.0)(&command_buffer) {
                Some(command_length) => *required_length = Some(command_length),
                // We don't have enough data yet so try again later.
                None => return,
            }
        }

        required_length.unwrap()
    };


    // Check if we can execute the command.
    if resources.gpu.gp0_command_buffer.len() < required_length_value {
        return;
    }

    // Execute it.
    {
        let command_buffer_slice: &[u32] = unsafe {
            &(&resources.gpu.gp0_command_buffer as *const Vec<u32>).as_ref().unwrap()[0..required_length_value]
        };
        
        (command_handler.1)(resources, video_backend, command_buffer_slice);
    
        if (command_buffer_slice[0] >> 24) < 0xE0 {
            debug::trace_gp0_command_render(resources, video_backend);
        }
    }
    
    // Setup for the next one.
    resources.gpu.gp0_command_buffer.drain(0..required_length_value);
    resources.gpu.gp0_command_required_length = None;
}

fn get_command_handler(command_index: u8) -> (LengthFn, HandlerFn) {
    match command_index {
        0x00 => (command_gp0_impl::command_00_length, command_gp0_impl::command_00_handler),
        0x01 => (command_gp0_impl::command_01_length, command_gp0_impl::command_01_handler),
        0x02 => (command_gp0_impl::command_02_length, command_gp0_impl::command_02_handler),
        0x05 => (command_gp0_impl::command_05_length, command_gp0_impl::command_05_handler),
        0x06 => (command_gp0_impl::command_06_length, command_gp0_impl::command_06_handler),
        0x0c => (command_gp0_impl::command_0c_length, command_gp0_impl::command_0c_handler),
        0x28 => (command_gp0_impl::command_28_length, command_gp0_impl::command_28_handler),
        0x2C => (command_gp0_impl::command_2c_length, command_gp0_impl::command_2c_handler),
        0x2D => (command_gp0_impl::command_2d_length, command_gp0_impl::command_2d_handler),
        0x30 => (command_gp0_impl::command_30_length, command_gp0_impl::command_30_handler),
        0x38 => (command_gp0_impl::command_38_length, command_gp0_impl::command_38_handler),
        0x3C => (command_gp0_impl::command_3c_length, command_gp0_impl::command_3c_handler),
        0x50 => (command_gp0_impl::command_50_length, command_gp0_impl::command_50_handler),
        0x65 => (command_gp0_impl::command_65_length, command_gp0_impl::command_65_handler),
        0x6F => (command_gp0_impl::command_6f_length, command_gp0_impl::command_6f_handler),
        0x80 => (command_gp0_impl::command_80_length, command_gp0_impl::command_80_handler),
        0xA0 => (command_gp0_impl::command_a0_length, command_gp0_impl::command_a0_handler),
        0xC0 => (command_gp0_impl::command_c0_length, command_gp0_impl::command_c0_handler),
        0xE1 => (command_gp0_impl::command_e1_length, command_gp0_impl::command_e1_handler),
        0xE2 => (command_gp0_impl::command_e2_length, command_gp0_impl::command_e2_handler),
        0xE3 => (command_gp0_impl::command_e3_length, command_gp0_impl::command_e3_handler),
        0xE4 => (command_gp0_impl::command_e4_length, command_gp0_impl::command_e4_handler),
        0xE5 => (command_gp0_impl::command_e5_length, command_gp0_impl::command_e5_handler),
        0xE6 => (command_gp0_impl::command_e6_length, command_gp0_impl::command_e6_handler),
        _ => unimplemented!("Unknown GP0 command: 0x{:0X}", command_index),
    }
}