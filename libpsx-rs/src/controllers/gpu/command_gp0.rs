use log::debug;
use crate::backends::video::VideoBackend;
use crate::State;
use crate::types::bitfield::Bitfield;
use crate::types::color::*;
use crate::types::geometry::*;
use crate::controllers::gpu::data::*;
use crate::controllers::gpu::opengl;
use crate::resources::gpu::*;

pub struct CommandHandler {
    /// Determines the amount of words needed to process the command.
    pub length_fn: fn(&[u32]) -> Option<usize>,
    /// The handler logic for the command.
    pub handler_fn: fn(&State, &[u32]),
}

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;

    let fifo = &mut resources.gpu.gpu1810.gp0;
    let command_buffer = &mut resources.gpu.gp0_command_buffer;

    // Update the command buffer with any new incoming data.
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

    // Get the associated command handler.
    let command = command_buffer[0];
    let command_index = GP_CMD.extract_from(command) as u8;
    let command_handler = get_command_handler(command_index);

    // Try and get the required data length.
    let required_length = &mut resources.gpu.gp0_command_required_length;
    if required_length.is_none() {
        match (command_handler.length_fn)(&command_buffer) {
            Some(command_length) => *required_length = Some(command_length),
            None => return,
        }
    }

    // Check if we can execute the command.
    let required_length_value = required_length.unwrap();
    if command_buffer.len() < required_length_value {
        return;
    }

    // Execute it.
    (command_handler.handler_fn)(state, &command_buffer);
    
    // Setup for the next one.
    command_buffer.drain(0..required_length_value);
    *required_length = None;
}

fn get_command_handler(command_index: u8) -> &'static CommandHandler {
    match command_index {
        0x00 => &COMMAND_00,
        0x01 => &COMMAND_01,
        0x05 => &COMMAND_05,
        0x06 => &COMMAND_06,
        0x0c => &COMMAND_0C,
        0x28 => &COMMAND_28,
        0x2C => &COMMAND_2C,
        0x30 => &COMMAND_30,
        0x38 => &COMMAND_38,
        0x3C => &COMMAND_3C,
        0x50 => &COMMAND_50,
        0x6F => &COMMAND_6F,
        0x80 => &COMMAND_80,
        0xA0 => &COMMAND_A0,
        0xC0 => &COMMAND_C0,
        0xE1 => &COMMAND_E1,
        0xE2 => &COMMAND_E2,
        0xE3 => &COMMAND_E3,
        0xE4 => &COMMAND_E4,
        0xE5 => &COMMAND_E5,
        0xE6 => &COMMAND_E6,
        _ => unimplemented!("Unknown GP0 command: 0x{:0X}", command_index),
    }
}

const COMMAND_00: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |_state: &State, _data: &[u32]| {
        // NOP
    }
};

const COMMAND_01: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |_state: &State, _data: &[u32]| {
        // Flush cache (NOP)
    }
};

const COMMAND_05: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |_state: &State, _data: &[u32]| {
        // NOP
    }
};

const COMMAND_06: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |_state: &State, _data: &[u32]| {
        // NOP
    }
};

const COMMAND_0C: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |_state: &State, _data: &[u32]| {
        // NOP
    }
};

const COMMAND_28: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(5)
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Monochrome four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4]);
        
        let color = extract_color_rgb(data[0], std::u8::MAX);
        let vertices = extract_vertices_4_normalized([data[1], data[2], data[3], data[4]]);
        
        match state.video_backend {
            VideoBackend::Opengl(ref backend_params) => {
                opengl::draw_polygon_4_solid(backend_params, vertices, color);
            },
        }
    }
};

const COMMAND_2C: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(9)
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]);

        // TODO: implement this properly - need to make a shader to do this I think...
        // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

        let _color = extract_color_rgb(data[0], std::u8::MAX);
        let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]]);
        let clut_mode = extract_texpage_clut_mode(data[4]);
        let _transparency_mode = extract_texpage_transparency_mode(data[4]);
        let texcoords = extract_texcoords_4_normalized(data[4], clut_mode, [data[2], data[4], data[6], data[8]]);
        let _clut = extract_clut_base_normalized(data[2]);

        match state.video_backend {
            VideoBackend::Opengl(ref backend_params) => {
                opengl::draw_polygon_4_textured_framebuffer(backend_params, vertices, texcoords);
            },
        }
    }
};

const COMMAND_30: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(6)
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Shaded three-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5]);

        let colors = extract_colors_3_rgb([data[0], data[2], data[4]], std::u8::MAX);
        let vertices = extract_vertices_3_normalized([data[1], data[3], data[5]]);

        match state.video_backend {
            VideoBackend::Opengl(ref backend_params) => {
                opengl::draw_polygon_3_shaded(backend_params, vertices, colors);
            },
        }
    }
};

const COMMAND_38: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(8)
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Shaded four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]);
        
        let colors = extract_colors_4_rgb([data[0], data[2], data[4], data[6]], std::u8::MAX);
        let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]]);

        match state.video_backend {
            VideoBackend::Opengl(ref backend_params) => {
                opengl::draw_polygon_4_shaded(backend_params, vertices, colors);
            },
        }
    }
};

const COMMAND_3C: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(12)
    },
    handler_fn: |_state: &State, data: &[u32]| {
        debug!("Shaded Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}, c10 = 0x{:0X}, c11 = 0x{:0X}, c12 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11]);
    }
};

const COMMAND_50: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(4)
    },
    handler_fn: |_state: &State, data: &[u32]| {
        debug!("Shaded line, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
    }
};

const COMMAND_6F: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(3)
    },
    handler_fn: |_state: &State, data: &[u32]| {
        debug!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);
    }
};

const COMMAND_80: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(4)
    },
    handler_fn: |_state: &State, data: &[u32]| {
        debug!("Copy Rectangle (VRAM to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
    }
};

const COMMAND_A0: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        if data.len() < 3 { 
            return None; 
        }

        let width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
        let height = Bitfield::new(16, 16).extract_from(data[2]) as usize;
        let count = width * height;
        let data_words = 3 + ((count + 1) / 2);

        return Some(data_words);
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Copy Rectangle (CPU to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, data len = {}", data[0], data[1], data[2], data.len() - 3);
        
        let base_point = extract_point_normalized(data[1]);
        let size = extract_size_normalized(data[2]);
        let texture_width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
        let texture_height = Bitfield::new(16, 16).extract_from(data[2]) as usize;

        let positions = [
            Point2D::new(base_point.x, base_point.y),
            Point2D::new(base_point.x + size.width, base_point.y),
            Point2D::new(base_point.x, base_point.y - size.height),
            Point2D::new(base_point.x + size.width, base_point.y - size.height),
        ];

        let texcoords = [
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(1.0, 1.0),
        ];

        // TODO: This is not a proper way to implement this command - the halfwords do not strictly represent pixels (16-bit colors / 5-5-5-1 colors).
        // However, the command addresses the VRAM (and incoming data) as 16-bit units through the coordinates given.
        // Ie: they could be 24-bit pixel data where 8-bits overflows into next pixel and so on... gives correct result for now. Hope this makes sense.... :/

        let mut texture_colors = Vec::with_capacity((data.len() - 3) * 2);
        let color_bitfields = [Bitfield::new(0, 16), Bitfield::new(16, 16)];
        for i in 3..data.len() {
            for field in color_bitfields.iter() {
                let packed_16 = field.extract_from(data[i]);
                let r = ((Bitfield::new(0, 5).extract_from(packed_16) * 255) / 31) as u8;
                let g = ((Bitfield::new(5, 5).extract_from(packed_16) * 255) / 31) as u8;
                let b = ((Bitfield::new(10, 5).extract_from(packed_16) * 255) / 31) as u8;
                //let mask = if Bitfield::new(15, 1).extract_from(packed_16) != 0 { std::u8::MAX } else { 0 };
                let mask = std::u8::MAX;
                texture_colors.push(Color::new(r, g, b, mask));
            }
        }

        match state.video_backend {
            VideoBackend::Opengl(ref backend_params) => {
                opengl::draw_polygon_4_textured(backend_params, positions, texcoords, texture_width, texture_height, &texture_colors);
            },
        }
    }
};

const COMMAND_C0: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(3)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            //debug!("Copy Rectangle (VRAM to CPU), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);

            let origin = Point2D::new(
                Bitfield::new(0, 16).extract_from(data[1]) as usize,
                Bitfield::new(16, 16).extract_from(data[1]) as usize,
            );

            let size = Size2D::new(
                Bitfield::new(0, 16).extract_from(data[2]) as usize,
                Bitfield::new(16, 16).extract_from(data[2]) as usize,
            );

            let count = size.width * size.height;

            if count == 0 {
                panic!("Empty size - what happens? ({:?})", size);
            }

            let fifo_words = (count + 1) / 2;

            let resources = &mut *state.resources;
            
            let mut data = match state.video_backend {
                VideoBackend::Opengl(ref backend_params) => {
                    opengl::read_framebuffer_5551(backend_params, origin, size)
                },
            };

            // Data is to be packed from 2 x u16 into u32.
            // Pad the last u16 if its an odd amount.
            if data.len() % 2 != 0 {
                data.push(0);
            }

            let read_buffer = &mut resources.gpu.gp0_read_buffer;
            for i in 0..fifo_words {
                let word: u32 = 0;
                Bitfield::new(0, 16).insert_into(word, data[i * 2] as u32);
                Bitfield::new(16, 16).insert_into(word, data[i * 2 + 1] as u32);
                read_buffer.push_back(word)
            }

            if size.height > 1 {
                unimplemented!("Verify function works properly for multiple lines");
            }
        }
    }
};

pub const COMMAND_E1: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            let stat = &mut resources.gpu.gpu1814.stat;
            stat.write_bitfield(STAT_TEXPAGEX, Bitfield::new(0, 4).extract_from(data[0]));
            stat.write_bitfield(STAT_TEXPAGEY, Bitfield::new(4, 1).extract_from(data[0]));
            stat.write_bitfield(STAT_TRANSPARENCY, Bitfield::new(5, 2).extract_from(data[0]));
            stat.write_bitfield(STAT_TEXPAGE_COLORS, Bitfield::new(7, 2).extract_from(data[0]));
            stat.write_bitfield(STAT_DITHER, Bitfield::new(9, 1).extract_from(data[0]));
            stat.write_bitfield(STAT_DRAW_DISPLAY, Bitfield::new(10, 1).extract_from(data[0]));
            stat.write_bitfield(STAT_TEXTURE_DISABLE, Bitfield::new(11, 1).extract_from(data[0]));
            resources.gpu.textured_rect_x_flip = Bitfield::new(12, 1).extract_from(data[0]) != 0;
            resources.gpu.textured_rect_y_flip = Bitfield::new(13, 1).extract_from(data[0]) != 0;
        }
    }
};

pub const COMMAND_E2: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            resources.gpu.texture_window_mask_x = Bitfield::new(0, 5).extract_from(data[0]) as usize;
            resources.gpu.texture_window_mask_y = Bitfield::new(5, 5).extract_from(data[0]) as usize;
            resources.gpu.texture_window_offset_x = Bitfield::new(10, 5).extract_from(data[0]) as usize;
            resources.gpu.texture_window_offset_y = Bitfield::new(15, 5).extract_from(data[0]) as usize;
        }
    }
};

pub const COMMAND_E3: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            resources.gpu.drawing_area_x1 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
            resources.gpu.drawing_area_y1 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
        }
    }
};

pub const COMMAND_E4: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            resources.gpu.drawing_area_x2 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
            resources.gpu.drawing_area_y2 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
        }
    }
};

pub const COMMAND_E5: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            resources.gpu.drawing_offset_x = Bitfield::new(0, 11).extract_from(data[0]) as usize;
            resources.gpu.drawing_offset_y = Bitfield::new(11, 11).extract_from(data[0]) as usize;
        }
    }
};

pub const COMMAND_E6: CommandHandler = CommandHandler {
    length_fn: |_data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, data: &[u32]| {
        unsafe {
            let resources = &mut *state.resources;
            let stat = &mut resources.gpu.gpu1814.stat;
            stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(data[0]));
            stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(data[0]));
        }
    }
};
