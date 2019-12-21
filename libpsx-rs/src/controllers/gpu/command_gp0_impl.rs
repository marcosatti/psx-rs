//use log::warn;
use crate::types::bitfield::Bitfield;
use crate::types::color::Color;
use crate::types::geometry::*;
use crate::resources::Resources;
use crate::resources::gpu::*;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::opengl;
use crate::controllers::gpu::data::*;
use crate::controllers::gpu::debug;

pub fn command_00_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_00_handler(_resources: &mut Resources, _video_backend: &VideoBackend, _data: &[u32]) {
}

pub fn command_01_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_01_handler(_resources: &mut Resources, _video_backend: &VideoBackend, _data: &[u32]) {
    // Flush cache (NOP)
}

pub fn command_02_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub fn command_02_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Fill Rectangle in VRAM", data);

    let color = extract_color_rgb(data[0], std::u8::MAX);
    // Upper left corner is starting point.
    let base_point = extract_point_normalized(data[1], default_fill_x_position_modifier, default_fill_y_position_modifier);
    let size = extract_size_normalized(data[2], default_fill_x_size_modifier, default_fill_y_size_modifier);

    let positions = [
        Point2D::new(base_point.x, base_point.y),
        Point2D::new(base_point.x + size.width, base_point.y),
        Point2D::new(base_point.x, base_point.y - size.height),
        Point2D::new(base_point.x + size.width, base_point.y - size.height),
    ];

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_solid(backend_params, positions, color);
        },
    }
}

pub fn command_05_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_05_handler(_resources: &mut Resources, _video_backend: &VideoBackend, _data: &[u32]) {
    // NOP
}

pub fn command_06_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_06_handler(_resources: &mut Resources, _video_backend: &VideoBackend, _data: &[u32]) {
    // NOP
}

pub fn command_0c_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_0c_handler(_resources: &mut Resources, _video_backend: &VideoBackend, _data: &[u32]) {
    // NOP
}

pub fn command_28_length(_data: &[u32]) -> Option<usize> {
    Some(5)
}

pub fn command_28_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Monochrome four-point polygon, opaque", data);
    
    let color = extract_color_rgb(data[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[2], data[3], data[4]], default_render_x_position_modifier, default_render_y_position_modifier);
    
    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_solid(backend_params, vertices, color);
        },
    }
}

pub fn command_2c_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub fn command_2c_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Textured four-point polygon, opaque, texture-blending", data);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let _color = extract_color_rgb(data[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let _transparency_mode = extract_texpage_transparency_mode(data[4]);
    let texcoords = extract_texcoords_4_normalized(data[4], clut_mode, [data[2], data[4], data[6], data[8]]);
    let _clut = extract_clut_base_normalized(data[2]);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_textured_framebuffer(backend_params, vertices, texcoords);
        },
    }
}

pub fn command_30_length(_data: &[u32]) -> Option<usize> {
    Some(6)
}

pub fn command_30_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Shaded three-point polygon, opaque", data);

    let colors = extract_colors_3_rgb([data[0], data[2], data[4]], std::u8::MAX);
    let vertices = extract_vertices_3_normalized([data[1], data[3], data[5]], default_render_x_position_modifier, default_render_y_position_modifier);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_3_shaded(backend_params, vertices, colors);
        },
    }
}

pub fn command_38_length(_data: &[u32]) -> Option<usize> {
    Some(8)
}

pub fn command_38_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Shaded four-point polygon, opaque", data);
    
    let colors = extract_colors_4_rgb([data[0], data[2], data[4], data[6]], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_shaded(backend_params, vertices, colors);
        },
    }
}

pub fn command_3c_length(_data: &[u32]) -> Option<usize> {
    Some(12)
}

pub fn command_3c_handler(_resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Shaded Textured four-point polygon, opaque, texture-blending", data);
}

pub fn command_50_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub fn command_50_handler(_resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Shaded line, opaque", data);
}

pub fn command_6f_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub fn command_6f_handler(_resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture", data);
}

pub fn command_80_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub fn command_80_handler(_resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Copy Rectangle (VRAM to VRAM)", data);
}

pub fn command_a0_length(data: &[u32]) -> Option<usize> {
    if data.len() < 3 { 
        return None; 
    }

    let width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(data[2]) as usize;
    let count = width * height;
    let data_words = 3 + ((count + 1) / 2);

    return Some(data_words);
}

pub fn command_a0_handler(_resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Copy Rectangle (CPU to VRAM)", data);
    
    let base_point = extract_point_normalized(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let size = extract_size_normalized(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);
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

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_textured(backend_params, positions, texcoords, texture_width, texture_height, &texture_colors);
        },
    }
}

pub fn command_c0_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub fn command_c0_handler(resources: &mut Resources, video_backend: &VideoBackend, data: &[u32]) {
    debug::trace_gp0_command("Copy Rectangle (VRAM to CPU)", data);

    let origin = extract_point(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let size = extract_size(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);

    let count = size.width * size.height;

    if count == 0 {
        panic!("Empty size - what happens? ({:?})", size);
    }

    let fifo_words = (count + 1) / 2;

    let mut data = match video_backend {
        VideoBackend::None => { unimplemented!() },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::read_framebuffer_5551(backend_params, origin, size)
        },
    };

    // Data is to be packed from 2 x u16 into u32.
    // Pad the last u16 if its an odd amount.
    if data.len() % 2 != 0 {
        data.push(0);
    }

    resources.gpu.gpu1810.read.clear();

    let read_buffer = &mut resources.gpu.gp0_read_buffer;
    read_buffer.clear();

    for i in 0..fifo_words {
        let mut word: u32 = 0;
        word = Bitfield::new(0, 16).insert_into(word, data[(i * 2) as usize] as u32);
        word = Bitfield::new(16, 16).insert_into(word, data[(i * 2 + 1) as usize] as u32);
        read_buffer.push_back(word)
    }
}

pub fn command_e1_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e1_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
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
    //warn!("GP0(E1h) not properly implemented");
}

pub fn command_e2_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e2_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    resources.gpu.texture_window_mask_x = Bitfield::new(0, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_mask_y = Bitfield::new(5, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_offset_x = Bitfield::new(10, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_offset_y = Bitfield::new(15, 5).extract_from(data[0]) as usize;
    //warn!("GP0(E2h) not properly implemented");
}

pub fn command_e3_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e3_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    resources.gpu.drawing_area_x1 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    resources.gpu.drawing_area_y1 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
    //warn!("GP0(E3h) not properly implemented");

    if (resources.gpu.drawing_area_x1 != 0) || (resources.gpu.drawing_area_y1 != 0) {
        log::debug!("Non zero drawing area x1 y1: {}, {}", resources.gpu.drawing_area_x1, resources.gpu.drawing_area_y1);
    }
}

pub fn command_e4_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e4_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    resources.gpu.drawing_area_x2 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    resources.gpu.drawing_area_y2 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
    //warn!("GP0(E4h) not properly implemented");

    if (resources.gpu.drawing_area_x2 != 0) || (resources.gpu.drawing_area_y2 != 0) {
        //log::debug!("Non zero drawing area x2 y2: {}, {}", resources.gpu.drawing_area_x2, resources.gpu.drawing_area_y2);
    }
}

pub fn command_e5_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e5_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    let x_offset = Bitfield::new(0, 11).extract_from(data[0]) as i16;
    let y_offset = Bitfield::new(11, 11).extract_from(data[0]) as i16;
    //warn!("GP0(E5h) not properly implemented");

    // Sign extend from 11-bit to 16-bit.
    resources.gpu.drawing_offset_x = ((x_offset << 5) >> 5) as isize;
    resources.gpu.drawing_offset_y = ((y_offset << 5) >> 5) as isize;
    
    //debug!("Drawing offset set to X = {} (raw = 0x{:X}), Y = {} (raw = 0x{:X})", x_offset, resources.gpu.drawing_offset_x, y_offset, resources.gpu.drawing_offset_y);
    if (x_offset != 0) || (y_offset != 0) {
        log::debug!("Non zero drawing offset: {}, {}", resources.gpu.drawing_offset_x, resources.gpu.drawing_offset_y);
    }
}

pub fn command_e6_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e6_handler(resources: &mut Resources, _video_backend: &VideoBackend, data: &[u32]) {
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(data[0]));
    //warn!("GP0(E6h) not properly implemented");
}
