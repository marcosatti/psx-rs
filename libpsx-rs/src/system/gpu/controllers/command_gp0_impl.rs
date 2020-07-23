use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            controllers::{
                backend_dispatch,
                data::*,
                debug,
            },
            types::ControllerState,
        },
        types::{
            ControllerResult,
            State,
        },
    },
    types::{
        bitfield::Bitfield,
        color::Color,
        geometry::*,
    },
    utilities::array::flip_rows,
};

pub(crate) fn command_00_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_00_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, _data: &[u32]) -> ControllerResult<()> {
    Ok(())
}

pub(crate) fn command_01_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_01_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, _data: &[u32]) -> ControllerResult<()> {
    // Flush cache (NOP)
    Ok(())
}

pub(crate) fn command_02_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_02_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Fill Rectangle in VRAM", data);

    let color = extract_color_rgb(data[0], std::u8::MAX);
    // Upper left corner is starting point.
    let base_point = extract_point_normalized(data[1], default_fill_x_position_modifier, default_fill_y_position_modifier);
    let size = extract_size_normalized(data[2], default_fill_x_size_modifier, default_fill_y_size_modifier);

    let positions = [
        Point2D::new(base_point.x, base_point.y - size.height),
        Point2D::new(base_point.x + size.width, base_point.y - size.height),
        Point2D::new(base_point.x, base_point.y),
        Point2D::new(base_point.x + size.width, base_point.y),
    ];

    let _ = backend_dispatch::draw_polygon_4_solid(video_backend, positions, color)?;

    Ok(())
}

pub(crate) fn command_05_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_05_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, _data: &[u32]) -> ControllerResult<()> {
    // NOP
    Ok(())
}

pub(crate) fn command_06_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_06_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, _data: &[u32]) -> ControllerResult<()> {
    // NOP
    Ok(())
}

pub(crate) fn command_0c_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_0c_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, _data: &[u32]) -> ControllerResult<()> {
    // NOP
    Ok(())
}

pub(crate) fn command_20_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_20_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome three-point polygon, opaque", data);

    let color = extract_color_rgb(data[0], std::u8::MAX);
    let positions = extract_vertices_3_normalized([data[1], data[2], data[3]], default_render_x_position_modifier, default_render_y_position_modifier);

    let _ = backend_dispatch::draw_polygon_3_solid(video_backend, positions, color)?;

    Ok(())
}

pub(crate) fn command_28_length(_data: &[u32]) -> Option<usize> {
    Some(5)
}

pub(crate) fn command_28_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome four-point polygon, opaque", data);

    let color = extract_color_rgb(data[0], std::u8::MAX);
    let positions = extract_vertices_4_normalized([data[1], data[2], data[3], data[4]], default_render_x_position_modifier, default_render_y_position_modifier);

    let _ = backend_dispatch::draw_polygon_4_solid(video_backend, positions, color)?;

    Ok(())
}

pub(crate) fn command_2c_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub(crate) fn command_2c_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured four-point polygon, opaque, texture-blending", data);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let _color = extract_color_rgb(data[0], std::u8::MAX);
    let positions = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let _transparency_mode = extract_texpage_transparency_mode(data[4]);
    let texcoords = extract_texcoords_4_normalized(data[4], clut_mode, [data[2], data[4], data[6], data[8]]);
    let _clut = extract_clut_base_normalized(data[2]);

    let _ = backend_dispatch::draw_polygon_4_textured_framebuffer(video_backend, positions, texcoords)?;

    Ok(())
}

pub(crate) fn command_2d_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub(crate) fn command_2d_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured four-point polygon, opaque, raw-texture", data);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let positions = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let _transparency_mode = extract_texpage_transparency_mode(data[4]);
    let texcoords = extract_texcoords_4_normalized(data[4], clut_mode, [data[2], data[4], data[6], data[8]]);
    let _clut = extract_clut_base_normalized(data[2]);

    let _ = backend_dispatch::draw_polygon_4_textured_framebuffer(video_backend, positions, texcoords)?;

    Ok(())
}

pub(crate) fn command_30_length(_data: &[u32]) -> Option<usize> {
    Some(6)
}

pub(crate) fn command_30_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded three-point polygon, opaque", data);

    let colors = extract_colors_3_rgb([data[0], data[2], data[4]], std::u8::MAX);
    let positions = extract_vertices_3_normalized([data[1], data[3], data[5]], default_render_x_position_modifier, default_render_y_position_modifier);

    let _ = backend_dispatch::draw_polygon_3_shaded(video_backend, positions, colors)?;

    Ok(())
}

pub(crate) fn command_38_length(_data: &[u32]) -> Option<usize> {
    Some(8)
}

pub(crate) fn command_38_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded four-point polygon, opaque", data);

    let colors = extract_colors_4_rgb([data[0], data[2], data[4], data[6]], std::u8::MAX);
    let positions = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);

    let _ = backend_dispatch::draw_polygon_4_shaded(video_backend, positions, colors)?;

    Ok(())
}

pub(crate) fn command_3c_length(_data: &[u32]) -> Option<usize> {
    Some(12)
}

pub(crate) fn command_3c_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded Textured four-point polygon, opaque, texture-blending", data);

    Ok(())
}

pub(crate) fn command_50_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_50_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded line, opaque", data);

    Ok(())
}

pub(crate) fn command_65_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_65_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured Rectangle, variable size, opaque, raw-texture", data);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let _color = extract_color_rgb(data[0], std::u8::MAX);
    // Upper left corner is starting point.
    let base_point = extract_point_normalized(data[1], default_render_x_position_modifier, default_render_y_position_modifier);
    let size = extract_size_normalized(data[3], default_render_x_size_modifier, default_render_y_size_modifier);
    let clut_mode = controller_state.clut_mode;
    let texpage_base = Point2D::new(controller_state.texpage_base_x, controller_state.texpage_base_y);
    let texcoords = extract_texcoords_rect_normalized(texpage_base, data[2], clut_mode, size);
    let texcoords = [texcoords[2], texcoords[3], texcoords[0], texcoords[1]];
    let _clut = extract_clut_base_normalized(data[2]);

    let positions: [Point2D<f32, Normalized>; 4] = [
        Point2D::new(base_point.x, base_point.y - size.height),
        Point2D::new(base_point.x + size.width, base_point.y - size.height),
        Point2D::new(base_point.x, base_point.y),
        Point2D::new(base_point.x + size.width, base_point.y),
    ];

    let _ = backend_dispatch::draw_polygon_4_textured_framebuffer(video_backend, positions, texcoords)?;

    Ok(())
}

pub(crate) fn command_6f_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_6f_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture", data);

    Ok(())
}

pub(crate) fn command_80_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_80_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Copy Rectangle (VRAM to VRAM)", data);

    Ok(())
}

pub(crate) fn command_a0_length(data: &[u32]) -> Option<usize> {
    if data.len() < 3 {
        return None;
    }

    let width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(data[2]) as usize;
    let count = width * height;
    let data_words = 3 + ((count + 1) / 2);

    return Some(data_words);
}

pub(crate) fn command_a0_handler(_state: &State, _controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Copy Rectangle (CPU to VRAM)", data);

    let base_point = extract_point_normalized(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let size = extract_size_normalized(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);
    let texture_width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
    let texture_height = Bitfield::new(16, 16).extract_from(data[2]) as usize;

    let positions: [Point2D<f32, Normalized>; 4] = [
        Point2D::new(base_point.x, base_point.y - size.height),
        Point2D::new(base_point.x + size.width, base_point.y - size.height),
        Point2D::new(base_point.x, base_point.y),
        Point2D::new(base_point.x + size.width, base_point.y),
    ];

    let texcoords: [Point2D<f32, Normalized>; 4] = [Point2D::new(0.0, 1.0), Point2D::new(1.0, 1.0), Point2D::new(0.0, 0.0), Point2D::new(1.0, 0.0)];

    // TODO: This is not a proper way to implement this command - the halfwords do not strictly represent pixels (16-bit
    // colors / 5-5-5-1 colors). However, the command addresses the VRAM (and incoming data) as 16-bit units through
    // the coordinates given. Ie: they could be 24-bit pixel data where 8-bits overflows into next pixel and so
    // on... gives correct result for now. Hope this makes sense.... :/

    let mut texture_colors = Vec::with_capacity((data.len() - 3) * 2);
    for i in 3..data.len() {
        let colors = Color::from_packed_5551_x2(data[i]);
        texture_colors.push(colors[0]);
        texture_colors.push(colors[1]);
    }

    let _ = backend_dispatch::draw_polygon_4_textured(video_backend, positions, texcoords, texture_width, texture_height, &texture_colors)?;

    Ok(())
}

pub(crate) fn command_c0_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_c0_handler(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Copy Rectangle (VRAM to CPU)", data);

    let pixel_origin = extract_point(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let pixel_size = extract_size(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);

    let count = pixel_size.width as usize * pixel_size.height as usize;
    assert!(count != 0, format!("Empty count (area) - what happens? ({:?})", pixel_size));
    let fifo_words = (count + 1) / 2;

    let mut origin = normalize_point(pixel_origin);
    let size = normalize_size(pixel_size);
    origin.y = origin.y - size.height;

    let data = backend_dispatch::read_framebuffer_5551(video_backend, origin, size)?.map_err(|_| "No backend available for reading framebuffer".to_owned())?;
    let mut data = flip_rows(&data, pixel_size.width as usize);
    assert!(data.len() == count, format!("Unexpected length of returned framebuffer rectangle buffer: expecting {}, got {}", count, data.len()));

    // Data is to be packed from 2 x u16 into u32. Pad the last u16 if its an odd amount.
    if data.len() % 2 != 0 {
        data.push(0);
    }

    state.gpu.read.clear();
    controller_state.gp0_read_buffer.clear();

    for i in 0..fifo_words {
        let mut word: u32 = 0;
        word = Bitfield::new(0, 16).insert_into(word, data[i * 2] as u32);
        word = Bitfield::new(16, 16).insert_into(word, data[i * 2 + 1] as u32);
        controller_state.gp0_read_buffer.push_back(word)
    }

    Ok(())
}

pub(crate) fn command_e1_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e1_handler(state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Draw Mode setting", data);

    let stat = &state.gpu.stat;

    let texpage_base_x = Bitfield::new(0, 4).extract_from(data[0]);
    stat.write_bitfield(STAT_TEXPAGEX, texpage_base_x);
    controller_state.texpage_base_x = texpage_base_x as isize;

    let texpage_base_y = Bitfield::new(4, 1).extract_from(data[0]);
    stat.write_bitfield(STAT_TEXPAGEY, texpage_base_y);
    controller_state.texpage_base_y = texpage_base_y as isize;

    let transparency_mode_raw = Bitfield::new(5, 2).extract_from(data[0]);
    stat.write_bitfield(STAT_TRANSPARENCY, transparency_mode_raw);
    controller_state.transparency_mode = extract_texpage_transparency_mode(data[0]);

    let clut_mode_raw = Bitfield::new(7, 2).extract_from(data[0]);
    stat.write_bitfield(STAT_TEXPAGE_COLORS, clut_mode_raw);
    controller_state.clut_mode = extract_texpage_clut_mode(data[0]);

    stat.write_bitfield(STAT_DITHER, Bitfield::new(9, 1).extract_from(data[0]));

    stat.write_bitfield(STAT_DRAW_DISPLAY, Bitfield::new(10, 1).extract_from(data[0]));

    stat.write_bitfield(STAT_TEXTURE_DISABLE, Bitfield::new(11, 1).extract_from(data[0]));

    controller_state.textured_rect_x_flip = Bitfield::new(12, 1).extract_from(data[0]) != 0;

    controller_state.textured_rect_y_flip = Bitfield::new(13, 1).extract_from(data[0]) != 0;
    // warn!("GP0(E1h) not properly implemented");

    Ok(())
}

pub(crate) fn command_e2_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e2_handler(_state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Texture Window setting", data);

    controller_state.texture_window_mask_x = Bitfield::new(0, 5).extract_from(data[0]) as usize;
    controller_state.texture_window_mask_y = Bitfield::new(5, 5).extract_from(data[0]) as usize;
    controller_state.texture_window_offset_x = Bitfield::new(10, 5).extract_from(data[0]) as isize;
    controller_state.texture_window_offset_y = Bitfield::new(15, 5).extract_from(data[0]) as isize;
    // warn!("GP0(E2h) not properly implemented");

    Ok(())
}

pub(crate) fn command_e3_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e3_handler(_state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Set Drawing Area top left", data);

    controller_state.drawing_area_x1 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    controller_state.drawing_area_y1 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
    // warn!("GP0(E3h) not properly implemented");

    if (controller_state.drawing_area_x1 != 0) || (controller_state.drawing_area_y1 != 0) {
        // log::debug!("Non zero drawing area x1 y1: {}, {}", controller_state.drawing_area_x1,
        // controller_state.drawing_area_y1);
    }

    Ok(())
}

pub(crate) fn command_e4_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e4_handler(_state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Set Drawing Area bottom right", data);

    controller_state.drawing_area_x2 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    controller_state.drawing_area_y2 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
    // warn!("GP0(E4h) not properly implemented");

    if (controller_state.drawing_area_x2 != 0) || (controller_state.drawing_area_y2 != 0) {
        // log::debug!("Non zero drawing area x2 y2: {}, {}", controller_state.drawing_area_x2,
        // controller_state.drawing_area_y2);
    }

    Ok(())
}

pub(crate) fn command_e5_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e5_handler(_state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Set Drawing Offset", data);

    let x_offset = Bitfield::new(0, 11).extract_from(data[0]) as i16;
    let y_offset = Bitfield::new(11, 11).extract_from(data[0]) as i16;
    // warn!("GP0(E5h) not properly implemented");

    // Sign extend from 11-bit to 16-bit.
    controller_state.drawing_offset_x = ((x_offset << 5) >> 5) as isize;
    controller_state.drawing_offset_y = ((y_offset << 5) >> 5) as isize;

    // debug!("Drawing offset set to X = {} (raw = 0x{:X}), Y = {} (raw = 0x{:X})", x_offset,
    // controller_state.drawing_offset_x, y_offset, controller_state.drawing_offset_y);
    if (x_offset != 0) || (y_offset != 0) {
        // log::debug!("Non zero drawing offset: {}, {}", controller_state.drawing_offset_x,
        // controller_state.drawing_offset_y);
    }

    Ok(())
}

pub(crate) fn command_e6_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub(crate) fn command_e6_handler(state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Mask Bit Setting", data);

    let stat = &state.gpu.stat;
    stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(data[0]));
    // warn!("GP0(E6h) not properly implemented");

    Ok(())
}
