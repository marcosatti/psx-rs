use crate::{
    system::gpu::{
        constants::*,
        types::rendering::*,
    },
    types::{
        color::*,
        geometry::*,
    },
};
use smallvec::SmallVec;

/// Calculates the normalized coordinates from given pixel coordinates.
/// Returned values are pixel-corner aligned (upper left corner). Corrections might need to be done as post-processing.
pub(crate) fn normalize_position(point: Point2D<isize, Pixel>) -> Point2D<f32, Normalized> {
    let position = Point2D::new(-1.0 + (2.0 * point.x as f32 / VRAM_WIDTH_16B as f32), 1.0 - (2.0 * point.y as f32 / VRAM_HEIGHT_LINES as f32));

    if false {
        if (position.x > 1.0) || (position.x < -1.0) || (position.y > 1.0) || (position.y < -1.0) {
            log::warn!("Position outside of valid range: {:?}", point);
        }
    }

    position
}

pub(crate) fn normalize_size(size: Size2D<isize, Pixel>) -> Size2D<f32, Normalized> {
    Size2D::new((size.width as f32 / VRAM_WIDTH_16B as f32) * 2.0, (size.height as f32 / VRAM_HEIGHT_LINES as f32) * 2.0)
}

/// Calculates the normalized texture coordinates from given pixel coordinates.
/// Returned values are pixel-corner aligned (upper left corner). Corrections might need to be done as post-processing.
pub(crate) fn normalize_texture_position(texcoord: Point2D<usize, Pixel>) -> Point2D<f32, NormalizedTexcoord> {
    let position = Point2D::new(texcoord.x as f32 / VRAM_WIDTH_16B as f32, 1.0 - (texcoord.y as f32 / VRAM_HEIGHT_LINES as f32));

    if false {
        if (position.x > 1.0) || (position.x < 0.0) || (position.y > 1.0) || (position.y < 0.0) {
            log::warn!("Texture position outside of valid range: {:?}", texcoord);
        }
    }

    position
}

pub(crate) fn normalize_texture_size(texcoord_size: Size2D<isize, Pixel>) -> Size2D<f32, NormalizedTexcoord> {
    Size2D::new(texcoord_size.width as f32 / VRAM_WIDTH_16B as f32, texcoord_size.height as f32 / VRAM_HEIGHT_LINES as f32)
}

pub(crate) fn normalized_to_texcoord_normalized(normalized: Point2D<f32, Normalized>) -> Point2D<f32, NormalizedTexcoord> {
    Point2D::new((normalized.x + 1.0) / 2.0, (normalized.y + 1.0) / 2.0)
}

pub(crate) fn normalized_to_texcoord_normalized_size(normalized_size: Size2D<f32, Normalized>) -> Size2D<f32, NormalizedTexcoord> {
    Size2D::new(normalized_size.width / 2.0, normalized_size.height / 2.0)
}

pub(crate) fn make_triangle_fan(rect: Rect<isize, Pixel>) -> [f32; 8] {
    let mut positions = [rect.origin; 4];
    // Upper left corner
    positions[0].x += 0;
    positions[0].y += 0;
    // Upper right corner
    positions[1].x += rect.size.width;
    positions[1].y += 0;
    // Lower right corner
    positions[2].x += rect.size.width;
    positions[2].y += rect.size.height;
    // Lower left corner
    positions[3].x += 0;
    positions[3].y += rect.size.height;

    let positions = [normalize_position(positions[0]), normalize_position(positions[1]), normalize_position(positions[2]), normalize_position(positions[3])];

    [positions[0].x, positions[0].y, positions[1].x, positions[1].y, positions[2].x, positions[2].y, positions[3].x, positions[3].y]
}

pub(crate) fn make_positions_normalized(positions: &[Point2D<isize, Pixel>]) -> SmallVec<[Point2D<f32, Normalized>; 4]> {
    positions.iter().map(|p| normalize_position(*p)).collect()
}

pub(crate) fn make_colors_normalized(colors: &[Color]) -> SmallVec<[NormalizedColor; 4]> {
    colors.iter().map(|c| c.to_normalized()).collect()
}

pub(crate) fn make_texture_position_offsets_normalized(texture_position_offsets: &[Size2D<isize, Pixel>]) -> SmallVec<[Size2D<f32, NormalizedTexcoord>; 4]> {
    texture_position_offsets.iter().map(|p| normalize_texture_size(*p)).collect()
}

pub(crate) fn rendering_mode_value(rendering_kind: RenderingKind) -> u32 {
    match rendering_kind {
        RenderingKind::Shaded => 0,
        RenderingKind::TextureBlending {
            ..
        } => 1,
        RenderingKind::RawTexture {
            ..
        } => 2,
    }
}

pub(crate) fn texture_position_base_value(rendering_kind: RenderingKind) -> [f32; 2] {
    match rendering_kind {
        RenderingKind::Shaded => [0.0, 0.0],
        RenderingKind::TextureBlending {
            page_base,
            ..
        } => normalize_texture_position(page_base).to_array(),
        RenderingKind::RawTexture {
            page_base,
            ..
        } => normalize_texture_position(page_base).to_array(),
    }
}

pub(crate) fn clut_mode_value(rendering_kind: RenderingKind) -> u32 {
    let clut_kind = match rendering_kind {
        RenderingKind::Shaded => {
            return 0;
        },
        RenderingKind::TextureBlending {
            clut_kind,
            ..
        } => clut_kind,
        RenderingKind::RawTexture {
            clut_kind,
            ..
        } => clut_kind,
    };

    match clut_kind {
        ClutKind::Direct => 0,
        ClutKind::Bits4 {
            ..
        } => 1,
        ClutKind::Bits8 {
            ..
        } => 2,
    }
}

pub(crate) fn clut_texture_position_base_value(rendering_kind: RenderingKind) -> [f32; 2] {
    let clut_kind = match rendering_kind {
        RenderingKind::Shaded => {
            return [0.0, 0.0];
        },
        RenderingKind::TextureBlending {
            clut_kind,
            ..
        } => clut_kind,
        RenderingKind::RawTexture {
            clut_kind,
            ..
        } => clut_kind,
    };

    match clut_kind {
        ClutKind::Direct => [0.0, 0.0],
        ClutKind::Bits4 {
            clut_base,
        } => normalize_texture_position(clut_base).to_array(),
        ClutKind::Bits8 {
            clut_base,
        } => normalize_texture_position(clut_base).to_array(),
    }
}

pub(crate) fn transparency_mode_value(transparency_kind: TransparencyKind) -> u32 {
    match transparency_kind {
        TransparencyKind::Opaque => 0,
        TransparencyKind::Average => 1,
        TransparencyKind::Additive => 2,
        TransparencyKind::Difference => 3,
        TransparencyKind::Quarter => 4,
    }
}
