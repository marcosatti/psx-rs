use crate::{backends::video::VideoBackend, system::{gpu::{constants::*, controllers::{
                backend_dispatch,
                data::*,
                debug,
            }, types::{
                rendering::*,
                ControllerState,
            }}, types::{
            ControllerResult,
            State,
        }}, types::{
        bitfield::Bitfield,
        color::*,
        geometry::*,
    }
};
use crate::utilities::bool_to_flag;

const NULL_TEXTURE_POSITION_OFFSET: Size2D<isize, Pixel> = Size2D::new(0, 0);
const NULL_TEXTURE_POSITION_OFFSET_3: [Size2D<isize, Pixel>; 3] = [NULL_TEXTURE_POSITION_OFFSET, NULL_TEXTURE_POSITION_OFFSET, NULL_TEXTURE_POSITION_OFFSET];
const NULL_TEXTURE_POSITION_OFFSET_4: [Size2D<isize, Pixel>; 4] = [NULL_TEXTURE_POSITION_OFFSET, NULL_TEXTURE_POSITION_OFFSET, NULL_TEXTURE_POSITION_OFFSET, NULL_TEXTURE_POSITION_OFFSET];
const NULL_COLOR: Color = Color::new(0, 0, 0);
const _NULL_COLOR_3: [Color; 3] = [NULL_COLOR, NULL_COLOR, NULL_COLOR];
const NULL_COLOR_4: [Color; 4] = [NULL_COLOR, NULL_COLOR, NULL_COLOR, NULL_COLOR];

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

    let origin = extract_position(data[1], default_fill_x_position_modifier, default_fill_y_position_modifier);
    let size = extract_size(data[2], default_fill_x_size_modifier, default_fill_y_size_modifier);
    let rectangle = Rect::new(origin, size);
    let color = extract_color(data[0]);

    let _ = backend_dispatch::draw_rectangle(video_backend, RectangleParams {
        rectangle,
        color,
        texture_position_base_offset: NULL_TEXTURE_POSITION_OFFSET,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: false,
        mask_bit_check: false,
    })?;

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

pub(crate) fn command_20_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome three-point polygon, opaque", data);

    let positions = extract_positions_3([data[1], data[2], data[3]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    
    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 3,
        positions: &positions,
        colors: &[color; 3],
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_3,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind:  TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_22_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_22_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome three-point polygon, semi-transparent", data);

    let positions = extract_positions_3([data[1], data[2], data[3]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    let transparency_kind = TransparencyKind::from_data(controller_state.transparency_mode);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 3,
        positions: &positions,
        colors: &[color; 3],
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_3,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind:  transparency_kind,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_28_length(_data: &[u32]) -> Option<usize> {
    Some(5)
}

pub(crate) fn command_28_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome four-point polygon, opaque", data);

    let positions = extract_positions_4([data[1], data[2], data[3], data[4]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    
    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &[color; 4],
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_4,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind:  TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_2a_length(_data: &[u32]) -> Option<usize> {
    Some(5)
}

pub(crate) fn command_2a_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Monochrome four-point polygon, semi-transparent", data);

    let positions = extract_positions_4([data[1], data[2], data[3], data[4]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    let transparency_kind = TransparencyKind::from_data(controller_state.transparency_mode);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &[color; 4],
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_4,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind:  transparency_kind,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_2c_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub(crate) fn command_2c_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured four-point polygon, opaque, texture-blending", data);

    let positions = extract_positions_4([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    let page_base = extract_texpage_base(data[4]);
    let texture_position_offsets = extract_texture_position_offsets_4([data[2], data[4], data[6], data[8]]);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::TextureBlending { page_base, clut_kind };

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &[color; 4],
        texture_position_offsets: &texture_position_offsets,
        rendering_kind,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_2d_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub(crate) fn command_2d_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured four-point polygon, opaque, raw-texture", data);

    let positions = extract_positions_4([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let page_base = extract_texpage_base(data[4]);
    let texture_position_offsets = extract_texture_position_offsets_4([data[2], data[4], data[6], data[8]]);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::RawTexture { page_base, clut_kind };

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &NULL_COLOR_4,
        texture_position_offsets: &texture_position_offsets,
        rendering_kind,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_2e_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub(crate) fn command_2e_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured four-point polygon, semi-transparent, texture-blending", data);

    let positions = extract_positions_4([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);
    let color = extract_color(data[0]);
    let page_base = extract_texpage_base(data[4]);
    let texture_position_offsets = extract_texture_position_offsets_4([data[2], data[4], data[6], data[8]]);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::TextureBlending { page_base, clut_kind };
    let transparency_kind = TransparencyKind::from_data(controller_state.transparency_mode);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &[color; 4],
        texture_position_offsets: &texture_position_offsets,
        rendering_kind,
        transparency_kind,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_30_length(_data: &[u32]) -> Option<usize> {
    Some(6)
}

pub(crate) fn command_30_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded three-point polygon, opaque", data);

    let positions = extract_positions_3([data[1], data[3], data[5]], default_render_x_position_modifier, default_render_y_position_modifier);
    let colors = extract_colors_3([data[0], data[2], data[4]]);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 3,
        positions: &positions,
        colors: &colors,
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_3,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_38_length(_data: &[u32]) -> Option<usize> {
    Some(8)
}

pub(crate) fn command_38_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded four-point polygon, opaque", data);

    let colors = extract_colors_4([data[0], data[2], data[4], data[6]]);
    let positions = extract_positions_4([data[1], data[3], data[5], data[7]], default_render_x_position_modifier, default_render_y_position_modifier);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &colors,
        texture_position_offsets: &NULL_TEXTURE_POSITION_OFFSET_4,
        rendering_kind: RenderingKind::Shaded,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_3c_length(_data: &[u32]) -> Option<usize> {
    Some(12)
}

pub(crate) fn command_3c_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded Textured four-point polygon, opaque, texture-blending", data);

    log::warn!("Shaded Textured four-point polygon, opaque, texture-blending not implemented");

    Ok(())
}

pub(crate) fn command_3e_length(_data: &[u32]) -> Option<usize> {
    Some(12)
}

pub(crate) fn command_3e_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded Textured four-point polygon, semi-transparent, tex-blend", data);

    let positions = extract_positions_4([data[1], data[4], data[7], data[10]], default_render_x_position_modifier, default_render_y_position_modifier);
    let colors = extract_colors_4([data[0], data[3], data[6], data[9]]);
    let page_base = extract_texpage_base(data[5]);
    let texture_position_offsets = extract_texture_position_offsets_4([data[2], data[5], data[8], data[11]]);
    let clut_mode = extract_texpage_clut_mode(data[5]);
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::TextureBlending { page_base, clut_kind };
    let transparency_kind = TransparencyKind::from_data(controller_state.transparency_mode);

    let _ = backend_dispatch::draw_triangles(video_backend, TrianglesParams {
        vertices: 4,
        positions: &positions,
        colors: &colors,
        texture_position_offsets: &texture_position_offsets,
        rendering_kind,
        transparency_kind,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_50_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_50_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Shaded line, opaque", data);

    log::warn!("Shaded line, opaque not implemented");

    Ok(())
}

pub(crate) fn command_65_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub(crate) fn command_65_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured Rectangle, variable size, opaque, raw-texture", data);

    let origin = extract_position(data[1], default_render_x_position_modifier, default_render_y_position_modifier);
    let size = extract_size(data[3], default_render_x_size_modifier, default_render_y_size_modifier);
    let rectangle = Rect::new(origin, size);
    let texture_position_base_offset = extract_texture_position_offset(data[2]);
    let page_base = Point2D::new(controller_state.texpage_base_x, controller_state.texpage_base_y);
    let clut_mode = controller_state.clut_mode;
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::RawTexture { page_base, clut_kind };

    let _ = backend_dispatch::draw_rectangle(video_backend, RectangleParams {
        rectangle,
        color: NULL_COLOR,
        texture_position_base_offset,
        rendering_kind,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_6f_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_6f_handler(_state: &State, _controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture", data);

    log::warn!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture not implemented");

    Ok(())
}

pub(crate) fn command_7c_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_7c_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Textured Rectangle, 16x16, opaque, texture-blending", data);

    let origin = extract_position(data[1], default_render_x_position_modifier, default_render_y_position_modifier);
    let size = Size2D::new(16, 16);
    let rectangle = Rect::new(origin, size);
    let color = extract_color(data[0]);
    let texture_position_base_offset = extract_texture_position_offset(data[2]);
    let page_base = Point2D::new(controller_state.texpage_base_x, controller_state.texpage_base_y);
    let clut_mode = controller_state.clut_mode;
    let clut_base = extract_clut_base(data[2]);
    let clut_kind = ClutKind::from_data(clut_mode, clut_base);
    let rendering_kind = RenderingKind::TextureBlending { page_base, clut_kind };

    let _ = backend_dispatch::draw_rectangle(video_backend, RectangleParams {
        rectangle,
        color,
        texture_position_base_offset,
        rendering_kind,
        transparency_kind: TransparencyKind::Opaque,
        mask_bit_force_set: controller_state.mask_bit_force_set,
        mask_bit_check: controller_state.mask_bit_check,
    })?;

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

pub(crate) fn command_a0_handler(_state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    // TODO: This is not a proper way to implement this command - the halfwords do not strictly represent pixels (16-bit
    // colors / 5-5-5-1 colors). However, the command addresses the VRAM (and incoming data) as 16-bit units through
    // the coordinates given. Ie: they could be 24-bit pixel data where 8-bits overflows into next pixel and so
    // on... gives correct result for now. Hope this makes sense.... :/
    // There is no great solution to this I don't think - just need to be careful about how data is interpreted including within the shaders.
    // (ie: uniform variable specifying 16-bit or 24-bit mode.)

    debug::trace_gp0_command("Copy Rectangle (CPU to VRAM)", data);

    let origin = extract_position(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let size = extract_size(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);
    let rectangle = Rect::new(origin, size);

    let mut texture_colors = Vec::with_capacity((data.len() - 3) * 2);
    for i in 3..data.len() {
        let colors = PackedColor::from_x2(data[i]);
        texture_colors.push(colors[0]);
        texture_colors.push(colors[1]);
    }

    let mask_bit_force_set = controller_state.mask_bit_force_set;
    let mask_bit_check = controller_state.mask_bit_check;

    let _ = backend_dispatch::write_framebuffer(video_backend, WriteFramebufferParams { 
        rectangle, 
        data: &texture_colors,
        mask_bit_force_set,
        mask_bit_check,
    })?;

    Ok(())
}

pub(crate) fn command_c0_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub(crate) fn command_c0_handler(state: &State, controller_state: &mut ControllerState, video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Copy Rectangle (VRAM to CPU)", data);

    let origin = extract_position(data[1], default_copy_x_position_modifier, default_copy_y_position_modifier);
    let size = extract_size(data[2], default_copy_x_size_modifier, default_copy_y_size_modifier);

    let count = size.width as usize * size.height as usize;
    assert!(count != 0, format!("Empty count (area) - what happens? ({:?})", size));

    let rectangle = Rect::new(origin, size);

    let mut data = backend_dispatch::read_framebuffer(video_backend, ReadFramebufferParams {
        rectangle
    })?.map_err(|_| "No backend available for reading framebuffer".to_owned())?;
    
    assert!(data.len() == count, format!("Unexpected length of returned framebuffer rectangle buffer: expecting {}, got {}", count, data.len()));

    // Data is to be packed from 2 x u16 into u32. Pad the last u16 if its an odd amount.
    if data.len() % 2 != 0 {
        data.push(PackedColor::new(0));
    }

    state.gpu.read.clear();
    controller_state.gp0_read_buffer.clear();

    let fifo_words = (count + 1) / 2;
    for i in 0..fifo_words {
        let mut word: u32 = 0;
        word = Bitfield::new(0, 16).insert_into(word, data[i * 2].color as u32);
        word = Bitfield::new(16, 16).insert_into(word, data[i * 2 + 1].color as u32);
        controller_state.gp0_read_buffer.push_back(word);
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

pub(crate) fn command_e6_handler(state: &State, controller_state: &mut ControllerState, _video_backend: &VideoBackend, data: &[u32]) -> ControllerResult<()> {
    debug::trace_gp0_command("Mask Bit Setting", data);

    let stat = &state.gpu.stat;

    let mask_bit_force_set = Bitfield::new(0, 1).extract_from(data[0]) > 0;
    let mask_bit_check = Bitfield::new(1, 1).extract_from(data[0]) > 0;

    controller_state.mask_bit_force_set = mask_bit_force_set;
    controller_state.mask_bit_check = mask_bit_check;

    stat.write_bitfield(STAT_DRAW_MASK, bool_to_flag(mask_bit_force_set));
    stat.write_bitfield(STAT_DRAW_PIXELS, bool_to_flag(mask_bit_check));
    // warn!("GP0(E6h) not properly implemented");

    Ok(())
}
