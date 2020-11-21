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

pub(crate) fn normalize_point(point: Point2D<isize, Pixel>) -> Point2D<f32, Normalized> {
    Point2D::new(-1.0 + ((2.0 / VRAM_WIDTH_16B as f32) * (point.x as f32)), 1.0 - ((2.0 / VRAM_HEIGHT_LINES as f32) * (point.y as f32)))
}

pub(crate) fn normalize_texcoord(texcoord: Point2D<isize, Pixel>) -> Point2D<f32, TexcoordNormalized> {
    // Accounts for half pixel offset (+0.5).
    Point2D::new((1.0 / VRAM_WIDTH_16B as f32) * (texcoord.x as f32 + 0.5), 1.0 - ((1.0 / VRAM_HEIGHT_LINES as f32) * (texcoord.y as f32 + 0.5)))
}

pub(crate) fn normalize_texcoord_size(texcoord_size: Size2D<isize, Pixel>) -> Size2D<f32, TexcoordNormalized> {
    Size2D::new(texcoord_size.width as f32 / VRAM_WIDTH_16B as f32, texcoord_size.height as f32 / VRAM_HEIGHT_LINES as f32)
}

pub(crate) fn normalized_to_texcoord_normalized(normalized: Point2D<f32, Normalized>) -> Point2D<f32, TexcoordNormalized> {
    // Accounts for half pixel offset (+0.5).
    Point2D::new(((normalized.x + (0.5 * 2.0 / VRAM_WIDTH_16B as f32)) + 1.0) / 2.0, ((normalized.y - (0.5 * 2.0 / VRAM_HEIGHT_LINES as f32)) + 1.0) / 2.0)
}

pub(crate) fn normalized_to_texcoord_normalized_size(normalized_size: Size2D<f32, Normalized>) -> Size2D<f32, TexcoordNormalized> {
    Size2D::new(normalized_size.width / 2.0, normalized_size.height / 2.0)
}

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
    Color::from_packed_888(color_raw, alpha)
}

pub(crate) fn extract_colors_3_rgb(colors_raw: [u32; 3], alpha: u8) -> [Color; 3] {
    [extract_color_rgb(colors_raw[0], alpha), extract_color_rgb(colors_raw[1], alpha), extract_color_rgb(colors_raw[2], alpha)]
}

pub(crate) fn extract_colors_4_rgb(colors_raw: [u32; 4], alpha: u8) -> [Color; 4] {
    [extract_color_rgb(colors_raw[0], alpha), extract_color_rgb(colors_raw[1], alpha), extract_color_rgb(colors_raw[2], alpha), extract_color_rgb(colors_raw[3], alpha)]
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

pub(crate) fn extract_texcoords_4_normalized(texpage_raw: u32, clut_mode: ClutMode, texcoords_raw: [u32; 4]) -> [Point2D<f32, TexcoordNormalized>; 4] {
    const TEXCOORD_X_BITFIELD: Bitfield = Bitfield::new(0, 8);
    const TEXCOORD_Y_BITFIELD: Bitfield = Bitfield::new(8, 8);

    let texpage = Bitfield::new(16, 16).extract_from(texpage_raw);
    let texpage_x_base = (Bitfield::new(0, 4).extract_from(texpage) * 64) as isize;
    let texpage_y_base = (Bitfield::new(4, 1).extract_from(texpage) * 256) as isize;
    let texpage_base = Point2D::new(texpage_x_base, texpage_y_base);

    // The raw texcoords are offsets given in terms of CLUT-mode-relative texture pixels, not framebuffer pixels.
    // For example, if the raw value of offset.x is 24, and we are using the 8-bit CLUT mode, that means the framebuffer
    // offset is 12 framebuffer pixels, since there are 2 framebuffer pixels per texture pixel in 8-bit CLUT mode. This
    // always assumes each framebuffer pixel is 16-bit.
    let texcoord_offsets: [Size2D<isize, Pixel>; 4] = [
        Size2D::new(TEXCOORD_X_BITFIELD.extract_from(texcoords_raw[0]) as isize, TEXCOORD_Y_BITFIELD.extract_from(texcoords_raw[0]) as isize),
        Size2D::new(TEXCOORD_X_BITFIELD.extract_from(texcoords_raw[1]) as isize, TEXCOORD_Y_BITFIELD.extract_from(texcoords_raw[1]) as isize),
        Size2D::new(TEXCOORD_X_BITFIELD.extract_from(texcoords_raw[2]) as isize, TEXCOORD_Y_BITFIELD.extract_from(texcoords_raw[2]) as isize),
        Size2D::new(TEXCOORD_X_BITFIELD.extract_from(texcoords_raw[3]) as isize, TEXCOORD_Y_BITFIELD.extract_from(texcoords_raw[3]) as isize),
    ];

    let scale_factor = match clut_mode {
        ClutMode::Bits4 => 16.0 / 4.0,
        ClutMode::Bits8 => 16.0 / 8.0,
        ClutMode::Bits15 => 1.0,
        _ => unimplemented!("Extracting texcoords CLUT mode unimplemented: {:?}", clut_mode),
    };

    let mut normalized_texcoords = [normalize_texcoord(texpage_base); 4];
    for i in 0..4 {
        let offset = normalize_texcoord_size(texcoord_offsets[i]);
        normalized_texcoords[i].x += offset.width / scale_factor;
        normalized_texcoords[i].y -= offset.height;
    }

    normalized_texcoords
}

pub(crate) fn extract_texcoords_rect_normalized(
    texpage_base: Point2D<isize, Pixel>, texcoord_offset_raw: u32, clut_mode: ClutMode, size: Size2D<f32, Normalized>,
) -> [Point2D<f32, TexcoordNormalized>; 4] {
    let texcoord_offset_x = Bitfield::new(0, 8).extract_from(texcoord_offset_raw) as isize;
    let texcoord_offset_y = Bitfield::new(8, 8).extract_from(texcoord_offset_raw) as isize;
    let texcoord_offset = Size2D::new(texcoord_offset_x, texcoord_offset_y);
    let texpage_base_normalized = normalize_texcoord(texpage_base) + normalize_texcoord_size(texcoord_offset);
    let texcoord_size = normalized_to_texcoord_normalized_size(size);

    let mut texcoords = [texpage_base_normalized; 4];

    let texcoord_offset_points: [Point2D<f32, TexcoordNormalized>; 4] =
        [Point2D::new(0.0, 0.0), Point2D::new(texcoord_size.width, 0.0), Point2D::new(0.0, texcoord_size.height), Point2D::new(texcoord_size.width, texcoord_size.height)];

    let scale_factor = match clut_mode {
        ClutMode::Bits4 => 16.0 / 4.0,
        ClutMode::Bits8 => 16.0 / 8.0,
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

pub(crate) fn extract_clut_base_texcoord_normalized(clut_raw: u32) -> Point2D<f32, TexcoordNormalized> {
    normalize_texcoord(extract_clut_base(clut_raw))
}
