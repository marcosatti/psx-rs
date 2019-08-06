use std::iter;
use log::debug;
use crate::backends::video::VideoBackend;
use crate::State;
use crate::types::bitfield::Bitfield;
use crate::types::color::*;
use crate::types::geometry::*;
use crate::controllers::gpu::data::*;
use crate::controllers::gpu::opengl;
use crate::resources::gpu::*;

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;

    let fifo = &mut resources.gpu.gpu1810.gp0;
    let command_queue = &mut resources.gpu.command_buffer;
    
    let command = fifo.peek_front();

    if let Some(value) = command {
        let cmd = GP_CMD.extract_from(value) as u8;

        match cmd {
            0x00 => command_00(fifo.read_one().unwrap()),
            0x01 => command_01(fifo.read_one().unwrap()),
            0x05 => command_05(fifo.read_one().unwrap()),
            0x06 => command_06(fifo.read_one().unwrap()),
            0x0c => command_0c(fifo.read_one().unwrap()),
            0x28 => {
                if fifo.len() < 5 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_28(state, value);
            },
            0x2C => {
                if fifo.len() < 9 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_2c(state, value);
            }
            0x30 => {
                if fifo.len() < 6 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_30(state, value);
            },
            0x38 => {
                if fifo.len() < 8 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_38(state, value);
            },
            0x3C => {
                if fifo.len() < 12 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_3c(state, value);
            },
            0x50 => {
                if fifo.len() < 4 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_50(state, value);
            },
            0x6F => {
                if fifo.len() < 3 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_6f(state, value);
            },
            0x80 => {
                if fifo.len() < 4 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_80(state, value);
            },
            0xA0 => {
                if fifo.len() < 3 { return; }

                let header = [fifo[0], fifo[1], fifo[2]];
                let width = Bitfield::new(0, 16).extract_from(header[2]) as usize;
                let height = Bitfield::new(16, 16).extract_from(header[2]) as usize;
                let count = width * height;
                let fifo_words = (count + 1) / 2;

                if fifo.len() < (3 + fifo_words) { return; }

                let header = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                let data = fifo.read(fifo_words).unwrap();
                command_a0(state, header, &data);
            },
            0xC0 => {
                if fifo.len() < 3 { return; }
                let value = [fifo.read_one().unwrap(), fifo.read_one().unwrap(), fifo.read_one().unwrap()];
                command_c0(state, value);
            },
            0xE1 => command_e1(state, fifo.read_one().unwrap()),
            0xE2 => command_e2(state, fifo.read_one().unwrap()),
            0xE3 => command_e3(state, fifo.read_one().unwrap()),
            0xE4 => command_e4(state, fifo.read_one().unwrap()),
            0xE5 => command_e5(state, fifo.read_one().unwrap()),
            0xE6 => command_e6(state, fifo.read_one().unwrap()),
            _ => unimplemented!("Unknown GP0 command: 0x{:0X} (0x{:0X})", cmd, value),
        }
    }
}

fn command_00(_command: u32) {
    // NOP
    //debug!("NOP 00, c1 = 0x{:0X}", _command);
}

fn command_01(_command: u32) {
    // Flush cache (NOP)
    //debug!("Flush cache (NOP) 01, c1 = 0x{:0X}", _command);
}

fn command_05(_command: u32) {
    // NOP
    //debug!("NOP 05, c1 = 0x{:0X}", _command);
}

fn command_06(_command: u32) {
    // NOP
    //debug!("NOP 06, c1 = 0x{:0X}", _command);
}

fn command_0c(_command: u32) {
    // NOP
    //debug!("NOP 0C, c1 = 0x{:0X}", _command);
}

fn command_28(state: &State, values: [u32; 5]) {
    //debug!("Monochrome four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4]);
    
    let color = extract_color_rgb(values[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([values[1], values[2], values[3], values[4]]);
    
    match state.video_backend {
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_solid(backend_params, vertices, color);
        },
    }
}

fn command_2c(state: &State, values: [u32; 9]) {
    //debug!("Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8]);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let _color = extract_color_rgb(values[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([values[1], values[3], values[5], values[7]]);
    let clut_mode = extract_texpage_clut_mode(values[4]);
    let _transparency_mode = extract_texpage_transparency_mode(values[4]);
    let texcoords = extract_texcoords_4_normalized(values[4], clut_mode, [values[2], values[4], values[6], values[8]]);
    let _clut = extract_clut_base_normalized(values[2]);

    match state.video_backend {
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_textured_framebuffer(backend_params, vertices, texcoords);
        },
    }
}

fn command_30(state: &State, values: [u32; 6]) {
    //debug!("Shaded three-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5]);

    let colors = extract_colors_3_rgb([values[0], values[2], values[4]], std::u8::MAX);
    let vertices = extract_vertices_3_normalized([values[1], values[3], values[5]]);

    match state.video_backend {
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_3_shaded(backend_params, vertices, colors);
        },
    }
}

fn command_38(state: &State, values: [u32; 8]) {
    //debug!("Shaded four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]);
    
    let colors = extract_colors_4_rgb([values[0], values[2], values[4], values[6]], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([values[1], values[3], values[5], values[7]]);

    match state.video_backend {
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_shaded(backend_params, vertices, colors);
        },
    }
}

fn command_3c(_state: &State, values: [u32; 12]) {
    debug!("Shaded Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}, c10 = 0x{:0X}, c11 = 0x{:0X}, c12 = 0x{:0X}", values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11]);
}

fn command_50(_state: &State, values: [u32; 4]) {
    debug!("Shaded line, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", values[0], values[1], values[2], values[3]);
}

fn command_6f(_state: &State, values: [u32; 3]) {
    debug!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", values[0], values[1], values[2]);
}

fn command_80(_state: &State, values: [u32; 4]) {
    debug!("Copy Rectangle (VRAM to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", values[0], values[1], values[2], values[3]);
}

fn command_a0(state: &State, values: [u32; 3], data: &[u32]) {
    //debug!("Copy Rectangle (CPU to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, data len = {}", values[0], values[1], values[2], data.len());
    
    let base_point = extract_point_normalized(values[1]);
    let size = extract_size_normalized(values[2]);
    let texture_width = Bitfield::new(0, 16).extract_from(values[2]) as usize;
    let texture_height = Bitfield::new(16, 16).extract_from(values[2]) as usize;

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

    let mut texture_colors = Vec::with_capacity(data.len() * 2);
    for i in 0..data.len() {
        let color_bitfields = [Bitfield::new(0, 16), Bitfield::new(16, 16)];
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

unsafe fn command_c0(state: &State, values: [u32; 3]) {
    //debug!("Copy Rectangle (VRAM to CPU), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", values[0], values[1], values[2]);
    let width = Bitfield::new(0, 16).extract_from(values[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(values[2]) as usize;
    let count = width * height;
    let fifo_words = (count + 1) / 2;

    debug!("Pushing {} words of 0xFFFF_FFFF to GPUREAD [not implemented properly]", fifo_words);

    let resources = &mut *state.resources;
    let read = &mut resources.gpu.gpu1810.read;
    read.write(iter::repeat(0xFFFF_FFFF).take(fifo_words))
}

pub unsafe fn command_e1(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_TEXPAGEX, Bitfield::new(0, 4).extract_from(command));
    stat.write_bitfield(STAT_TEXPAGEY, Bitfield::new(4, 1).extract_from(command));
    stat.write_bitfield(STAT_TRANSPARENCY, Bitfield::new(5, 2).extract_from(command));
    stat.write_bitfield(STAT_TEXPAGE_COLORS, Bitfield::new(7, 2).extract_from(command));
    stat.write_bitfield(STAT_DITHER, Bitfield::new(9, 1).extract_from(command));
    stat.write_bitfield(STAT_DRAW_DISPLAY, Bitfield::new(10, 1).extract_from(command));
    stat.write_bitfield(STAT_TEXTURE_DISABLE, Bitfield::new(11, 1).extract_from(command));
    resources.gpu.textured_rect_x_flip = Bitfield::new(12, 1).extract_from(command) != 0;
    resources.gpu.textured_rect_y_flip = Bitfield::new(13, 1).extract_from(command) != 0;
}

pub unsafe fn command_e2(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.texture_window_mask_x = Bitfield::new(0, 5).extract_from(command) as usize;
    resources.gpu.texture_window_mask_y = Bitfield::new(5, 5).extract_from(command) as usize;
    resources.gpu.texture_window_offset_x = Bitfield::new(10, 5).extract_from(command) as usize;
    resources.gpu.texture_window_offset_y = Bitfield::new(15, 5).extract_from(command) as usize;
}

pub unsafe fn command_e3(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_area_x1 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.drawing_area_y1 = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub unsafe fn command_e4(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_area_x2 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.drawing_area_y2 = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub unsafe fn command_e5(state: &State, command: u32) {
    let resources = &mut *state.resources;
    resources.gpu.drawing_offset_x = Bitfield::new(0, 11).extract_from(command) as usize;
    resources.gpu.drawing_offset_y = Bitfield::new(11, 11).extract_from(command) as usize;
}

pub unsafe fn command_e6(state: &State, command: u32) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(command));
    stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(command));
}
