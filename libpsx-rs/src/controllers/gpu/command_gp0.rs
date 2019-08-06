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
    command_buffer.clear();
    *required_length = None;
}

fn get_command_handler(command_index: u8) -> &'static CommandHandler {
    match command_index {
        0x00 => &command_00,
        0x01 => &command_01,
        0x05 => &command_05,
        0x06 => &command_06,
        0x0c => &command_0c,
        0x28 => &command_28,
        0x2C => &command_2c,
        0x30 => &command_30,
        0x38 => &command_38,
        0x3C => &command_3c,
        0x50 => &command_50,
        0x6F => &command_6f,
        0x80 => &command_80,
        0xA0 => &command_a0,
        0xC0 => &command_c0,
        0xE1 => &command_e1,
        0xE2 => &command_e2,
        0xE3 => &command_e3,
        0xE4 => &command_e4,
        0xE5 => &command_e5,
        0xE6 => &command_e6,
        _ => unimplemented!("Unknown GP0 command: 0x{:0X}", command_index),
    }
}

static command_00: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, _data: &[u32]| {
        // NOP
    }
};

static command_01: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, _data: &[u32]| {
        // Flush cache (NOP)
    }
};

static command_05: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, _data: &[u32]| {
        // NOP
    }
};

static command_06: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, _data: &[u32]| {
        // NOP
    }
};

static command_0c: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(1)
    },
    handler_fn: |state: &State, _data: &[u32]| {
        // NOP
    }
};

static command_28: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

static command_2c: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

static command_30: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

static command_38: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

static command_3c: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(12)
    },
    handler_fn: |state: &State, data: &[u32]| {
        debug!("Shaded Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}, c10 = 0x{:0X}, c11 = 0x{:0X}, c12 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11]);
    }
};

static command_50: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(4)
    },
    handler_fn: |state: &State, data: &[u32]| {
        debug!("Shaded line, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
    }
};

static command_6f: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(3)
    },
    handler_fn: |state: &State, data: &[u32]| {
        debug!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);
    }
};

static command_80: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(4)
    },
    handler_fn: |state: &State, data: &[u32]| {
        debug!("Copy Rectangle (VRAM to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
    }
};

static command_a0: CommandHandler = CommandHandler {
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

static command_c0: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
        Some(3)
    },
    handler_fn: |state: &State, data: &[u32]| {
        //debug!("Copy Rectangle (VRAM to CPU), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);
        let width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
        let height = Bitfield::new(16, 16).extract_from(data[2]) as usize;
        let count = width * height;
        let fifo_words = (count + 1) / 2;

        debug!("Pushing {} words of 0xFFFF_FFFF to GPUREAD [not implemented properly]", fifo_words);

        // let resources = &mut *state.resources;
        // let read = &mut resources.gpu.gpu1810.read;
        // read.write(iter::repeat(0xFFFF_FFFF).take(fifo_words));
    }
};

pub static command_e1: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

pub static command_e2: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

pub static command_e3: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

pub static command_e4: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

pub static command_e5: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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

pub static command_e6: CommandHandler = CommandHandler {
    length_fn: |data: &[u32]| -> Option<usize> {
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
