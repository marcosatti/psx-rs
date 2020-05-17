use crate::{
    system::gpu::{
        constants::*,
        types::*,
    },
    types::{
        bitfield::Bitfield,
        color::Color,
        geometry::*,
    },
};

pub(crate) fn default_render_x_position_modifier(d: isize) -> isize {
    // Sign extend from 11-bit to isize.
    let d = Bitfield::new(0, 11).extract_from(d as i16);
    ((d << 5) >> 5) as isize
}

pub(crate) fn default_render_y_position_modifier(d: isize) -> isize {
    // Sign extend from 11-bit to isize.
    let d = Bitfield::new(0, 11).extract_from(d as i16);
    ((d << 5) >> 5) as isize
}

pub(crate) fn default_fill_x_position_modifier(d: isize) -> isize {
    d & 0x3F0
}

pub(crate) fn default_fill_y_position_modifier(d: isize) -> isize {
    d & 0x1FF
}

pub(crate) fn default_render_x_size_modifier(d: isize) -> isize {
    d
}

pub(crate) fn default_render_y_size_modifier(d: isize) -> isize {
    d
}

pub(crate) fn default_fill_x_size_modifier(d: isize) -> isize {
    ((d & 0x3FF) + 0xF) & (!0xF)
}

pub(crate) fn default_fill_y_size_modifier(d: isize) -> isize {
    d & 0x1FF
}

pub(crate) fn default_copy_x_position_modifier(d: isize) -> isize {
    d & 0x3FF
}

pub(crate) fn default_copy_y_position_modifier(d: isize) -> isize {
    d & 0x1FF
}

pub(crate) fn default_copy_x_size_modifier(d: isize) -> isize {
    ((d - 1) & 0x3FF) + 1
}

pub(crate) fn default_copy_y_size_modifier(d: isize) -> isize {
    ((d - 1) & 0x1FF) + 1
}

pub(crate) fn extract_texpage_transparency_mode(texpage_raw: u32) -> TransparencyMode {
    match Bitfield::new(21, 2).extract_from(texpage_raw) {
        0 => TransparencyMode::Average,
        1 => TransparencyMode::Additive,
        2 => TransparencyMode::Difference,
        3 => TransparencyMode::Quarter,
        _ => unreachable!("Invalid transparency mode"),
    }
}

pub(crate) fn extract_texpage_clut_mode(texpage_raw: u32) -> ClutMode {
    match Bitfield::new(23, 2).extract_from(texpage_raw) {
        0 => ClutMode::Bits4,
        1 => ClutMode::Bits8,
        2 => ClutMode::Bits15,
        3 => ClutMode::Reserved,
        _ => unreachable!("Invalid CLUT mode"),
    }
}

pub(crate) fn extract_color_rgb(color_raw: u32, alpha: u8) -> Color {
    Color::new(Bitfield::new(0, 8).extract_from(color_raw) as u8, Bitfield::new(8, 8).extract_from(color_raw) as u8, Bitfield::new(16, 8).extract_from(color_raw) as u8, alpha)
}

pub(crate) fn extract_colors_3_rgb(colors_raw: [u32; 3], alpha: u8) -> [Color; 3] {
    [extract_color_rgb(colors_raw[0], alpha), extract_color_rgb(colors_raw[1], alpha), extract_color_rgb(colors_raw[2], alpha)]
}

pub(crate) fn extract_colors_4_rgb(colors_raw: [u32; 4], alpha: u8) -> [Color; 4] {
    [extract_color_rgb(colors_raw[0], alpha), extract_color_rgb(colors_raw[1], alpha), extract_color_rgb(colors_raw[2], alpha), extract_color_rgb(colors_raw[3], alpha)]
}

pub(crate) fn normalize_point(point: Point2D<isize, Pixel>) -> Point2D<f32, Normalized> {
    Point2D::new(
        ((point.x as f64 - ((VRAM_WIDTH_16B as f64 / 2.0) - 1.0)) / ((VRAM_WIDTH_16B as f64 / 2.0) - 1.0)) as f32,
        (-((point.y as f64 - ((VRAM_HEIGHT_LINES as f64 / 2.0) - 1.0)) / ((VRAM_HEIGHT_LINES as f64 / 2.0) - 1.0))) as f32,
    )
}

pub(crate) fn extract_point(point_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Point2D<isize, Pixel> {
    Point2D::new(x_modifier(Bitfield::new(0, 16).extract_from(point_raw) as isize), y_modifier(Bitfield::new(16, 16).extract_from(point_raw) as isize))
}

pub(crate) fn extract_point_normalized(point_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Point2D<f32, Normalized> {
    normalize_point(extract_point(point_raw, x_modifier, y_modifier))
}

pub(crate) fn normalize_size(size: Size2D<isize, Pixel>) -> Size2D<f32, Normalized> {
    Size2D::new((size.width as f32 / VRAM_WIDTH_16B as f32) * 2.0, (size.height as f32 / VRAM_HEIGHT_LINES as f32) * 2.0)
}

pub(crate) fn extract_size(size_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Size2D<isize, Pixel> {
    Size2D::new(x_modifier(Bitfield::new(0, 16).extract_from(size_raw) as isize), y_modifier(Bitfield::new(16, 16).extract_from(size_raw) as isize))
}

pub(crate) fn extract_size_normalized(size_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Size2D<f32, Normalized> {
    normalize_size(extract_size(size_raw, x_modifier, y_modifier))
}

pub(crate) fn normalize_points_3(points: [Point2D<isize, Pixel>; 3]) -> [Point2D<f32, Normalized>; 3] {
    [normalize_point(points[0]), normalize_point(points[1]), normalize_point(points[2])]
}

pub(crate) fn extract_vertices_3(vertices_raw: [u32; 3], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Point2D<isize, Pixel>; 3] {
    [extract_point(vertices_raw[0], x_modifier, y_modifier), extract_point(vertices_raw[1], x_modifier, y_modifier), extract_point(vertices_raw[2], x_modifier, y_modifier)]
}

pub(crate) fn extract_vertices_3_normalized(vertices_raw: [u32; 3], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Point2D<f32, Normalized>; 3] {
    normalize_points_3(extract_vertices_3(vertices_raw, x_modifier, y_modifier))
}

pub(crate) fn normalize_points_4(points: [Point2D<isize, Pixel>; 4]) -> [Point2D<f32, Normalized>; 4] {
    [normalize_point(points[0]), normalize_point(points[1]), normalize_point(points[2]), normalize_point(points[3])]
}

pub(crate) fn extract_vertices_4(vertices_raw: [u32; 4], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Point2D<isize, Pixel>; 4] {
    [
        extract_point(vertices_raw[0], x_modifier, y_modifier),
        extract_point(vertices_raw[1], x_modifier, y_modifier),
        extract_point(vertices_raw[2], x_modifier, y_modifier),
        extract_point(vertices_raw[3], x_modifier, y_modifier),
    ]
}

pub(crate) fn extract_vertices_4_normalized(vertices_raw: [u32; 4], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Point2D<f32, Normalized>; 4] {
    normalize_points_4(extract_vertices_4(vertices_raw, x_modifier, y_modifier))
}

pub(crate) fn extract_texcoords_4_normalized(texpage_raw: u32, clut_mode: ClutMode, texcoords_raw: [u32; 4]) -> [Point2D<f32, Normalized>; 4] {
    let texpage = Bitfield::new(16, 16).extract_from(texpage_raw);
    let texpage_x_base = (Bitfield::new(0, 4).extract_from(texpage) * 64) as isize;
    let texpage_y_base = (Bitfield::new(4, 1).extract_from(texpage) * 256) as isize;
    let texpage_x_base = texpage_x_base as f32 / (VRAM_WIDTH_16B as f32 - 1.0);
    let texpage_y_base = 1.0 - (texpage_y_base as f32 / (VRAM_HEIGHT_LINES as f32 - 1.0));
    let texpage_base = Point2D::new(texpage_x_base, texpage_y_base);

    let mut texcoords: [Point2D<f32, Normalized>; 4] = [texpage_base; 4];

    let texcoord_x_bitfield = Bitfield::new(0, 8);
    let texcoord_y_bitfield = Bitfield::new(8, 8);

    // The texcoords are in terms of texture pixels, not framebuffer pixels (see clut_mode below).
    let texcoord_offset_points: [Point2D<isize, Pixel>; 4] = [
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[0]) as isize, texcoord_y_bitfield.extract_from(texcoords_raw[0]) as isize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[1]) as isize, texcoord_y_bitfield.extract_from(texcoords_raw[1]) as isize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[2]) as isize, texcoord_y_bitfield.extract_from(texcoords_raw[2]) as isize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[3]) as isize, texcoord_y_bitfield.extract_from(texcoords_raw[3]) as isize),
    ];

    // Each framebuffer pixel represents {scale_factor} number of texture pixels.
    let scale_factor = match clut_mode {
        ClutMode::Bits4 => 4.0,
        ClutMode::Bits8 => 2.0,
        ClutMode::Bits15 => 1.0,
        _ => unimplemented!("Extracting texcoords CLUT mode unimplemented: {:?}", clut_mode),
    };

    for i in 0..4 {
        texcoords[i].x += (texcoord_offset_points[i].x as f32 / scale_factor) / (VRAM_WIDTH_16B as f32 - 1.0);
        texcoords[i].y -= texcoord_offset_points[i].y as f32 / (VRAM_HEIGHT_LINES as f32 - 1.0);
    }

    texcoords
}

pub(crate) fn extract_texcoords_rect_normalized(
    texpage_base: Point2D<isize, Pixel>, texcoord_offset_raw: u32, clut_mode: ClutMode, size: Size2D<f32, Normalized>,
) -> [Point2D<f32, Normalized>; 4] {
    let texcoord_offset_x = Bitfield::new(0, 8).extract_from(texcoord_offset_raw) as isize;
    let texcoord_offset_y = Bitfield::new(8, 8).extract_from(texcoord_offset_raw) as isize;
    let texpage_x_base = (texpage_base.x + texcoord_offset_x) as f32 / (VRAM_WIDTH_16B as f32 - 1.0);
    let texpage_y_base = 1.0 - ((texpage_base.y + texcoord_offset_y) as f32 / (VRAM_HEIGHT_LINES as f32 - 1.0));
    let texpage_base = Point2D::new(texpage_x_base, texpage_y_base);

    let mut texcoords: [Point2D<f32, Normalized>; 4] = [texpage_base; 4];

    // The texcoords are in terms of texture pixels, not framebuffer pixels (see clut_mode below).
    let texcoord_offset_points: [Point2D<f32, Normalized>; 4] =
        [Point2D::new(0.0, 0.0), Point2D::new(size.width, 0.0), Point2D::new(0.0, size.height), Point2D::new(size.width, size.height)];

    // Each framebuffer pixel represents {scale_factor} number of texture pixels.
    let scale_factor = match clut_mode {
        ClutMode::Bits4 => 4.0,
        ClutMode::Bits8 => 2.0,
        ClutMode::Bits15 => 1.0,
        _ => unimplemented!("Extracting texcoords CLUT mode unimplemented: {:?}", clut_mode),
    };

    for i in 0..4 {
        texcoords[i].x += texcoord_offset_points[i].x / scale_factor;
        texcoords[i].y -= texcoord_offset_points[i].y;
    }

    texcoords
}

pub(crate) fn extract_clut_base(clut_raw: u32) -> Point2D<isize, Pixel> {
    let clut = Bitfield::new(16, 16).extract_from(clut_raw);
    let clut_x = (Bitfield::new(0, 6).extract_from(clut) * 16) as isize;
    let clut_y = Bitfield::new(6, 9).extract_from(clut) as isize;

    Point2D::new(clut_x, clut_y)
}

pub(crate) fn extract_clut_base_normalized(clut_raw: u32) -> Point2D<f32, Normalized> {
    normalize_point(extract_clut_base(clut_raw))
}
