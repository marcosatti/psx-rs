use crate::{
    system::gpu::{
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

pub(crate) fn extract_color(color_raw: u32) -> Color {
    const R: Bitfield = Bitfield::new(0, 8);
    const G: Bitfield = Bitfield::new(8, 8);
    const B: Bitfield = Bitfield::new(16, 8);

    Color::new(R.extract_from(color_raw) as u8, G.extract_from(color_raw) as u8, B.extract_from(color_raw) as u8)
}

pub(crate) fn extract_colors_3(colors_raw: [u32; 3]) -> [Color; 3] {
    [
        extract_color(colors_raw[0]), 
        extract_color(colors_raw[1]), 
        extract_color(colors_raw[2]),
    ]
}

pub(crate) fn extract_colors_4(colors_raw: [u32; 4]) -> [Color; 4] {
    [
        extract_color(colors_raw[0]), 
        extract_color(colors_raw[1]), 
        extract_color(colors_raw[2]), 
        extract_color(colors_raw[3]),
    ]
}

pub(crate) fn extract_position_offset(point_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Size2D<isize, Pixel> {
    Size2D::new(x_modifier(Bitfield::new(0, 16).extract_from(point_raw) as isize), y_modifier(Bitfield::new(16, 16).extract_from(point_raw) as isize))
}

pub(crate) fn extract_size(size_raw: u32, x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> Size2D<isize, Pixel> {
    Size2D::new(x_modifier(Bitfield::new(0, 16).extract_from(size_raw) as isize), y_modifier(Bitfield::new(16, 16).extract_from(size_raw) as isize))
}

pub(crate) fn extract_position_offsets_3(positions_raw: [u32; 3], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Size2D<isize, Pixel>; 3] {
    [
        extract_position_offset(positions_raw[0], x_modifier, y_modifier), 
        extract_position_offset(positions_raw[1], x_modifier, y_modifier), 
        extract_position_offset(positions_raw[2], x_modifier, y_modifier),
    ]
}

pub(crate) fn extract_position_offsets_4(positions_raw: [u32; 4], x_modifier: fn(isize) -> isize, y_modifier: fn(isize) -> isize) -> [Size2D<isize, Pixel>; 4] {
    [
        extract_position_offset(positions_raw[0], x_modifier, y_modifier),
        extract_position_offset(positions_raw[1], x_modifier, y_modifier),
        extract_position_offset(positions_raw[2], x_modifier, y_modifier),
        extract_position_offset(positions_raw[3], x_modifier, y_modifier),
    ]
}

pub(crate) fn extract_texture_position_offset(texcoord_raw: u32) -> Size2D<isize, Pixel> {
    Size2D::new(Bitfield::new(0, 8).extract_from(texcoord_raw) as isize, Bitfield::new(8, 8).extract_from(texcoord_raw) as isize)
}

pub(crate) fn extract_texture_position_offsets_3(texcoords_raw: [u32; 3]) -> [Size2D<isize, Pixel>; 3] {

    [
        extract_texture_position_offset(texcoords_raw[0]),
        extract_texture_position_offset(texcoords_raw[1]),
        extract_texture_position_offset(texcoords_raw[2]),
    ]
}

pub(crate) fn extract_texture_position_offsets_4(texcoords_raw: [u32; 4]) -> [Size2D<isize, Pixel>; 4] {

    [
        extract_texture_position_offset(texcoords_raw[0]),
        extract_texture_position_offset(texcoords_raw[1]),
        extract_texture_position_offset(texcoords_raw[2]),
        extract_texture_position_offset(texcoords_raw[3]),
    ]
}

pub(crate) fn extract_clut_base(clut_raw: u32) -> Point2D<usize, Pixel> {
    const CLUT: Bitfield = Bitfield::new(16, 16);
    const CLUT_X: Bitfield = Bitfield::new(0, 6);
    const CLUT_Y: Bitfield = Bitfield::new(6, 9);
    
    let clut = CLUT.extract_from(clut_raw);
    let clut_x = CLUT_X.extract_from(clut) as usize * 16;
    let clut_y = CLUT_Y.extract_from(clut) as usize;

    Point2D::new(clut_x, clut_y)
}

pub(crate) fn extract_texpage_base(texpage_raw: u32) -> Point2D<usize, Pixel> {
    const TEXPAGE: Bitfield = Bitfield::new(16, 16);
    const TEXPAGE_X: Bitfield = Bitfield::new(0, 4);
    const TEXPAGE_Y: Bitfield = Bitfield::new(4, 1);

    let texpage = TEXPAGE.extract_from(texpage_raw);
    let texpage_x_base = TEXPAGE_X.extract_from(texpage) as usize * 64;
    let texpage_y_base = TEXPAGE_Y.extract_from(texpage) as usize * 256;
    
    Point2D::new(texpage_x_base, texpage_y_base)
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

pub(crate) fn make_position(base: Point2D<isize, Pixel>, offset: Size2D<isize, Pixel>) -> Point2D<isize, Pixel> {
    base + offset
}

pub(crate) fn make_positions_3(base: Point2D<isize, Pixel>, offsets: [Size2D<isize, Pixel>; 3]) -> [Point2D<isize, Pixel>; 3] {
    [make_position(base, offsets[0]), make_position(base, offsets[1]), make_position(base, offsets[2])]
}

pub(crate) fn make_positions_4(base: Point2D<isize, Pixel>, offsets: [Size2D<isize, Pixel>; 4]) -> [Point2D<isize, Pixel>; 4] {
    [make_position(base, offsets[0]), make_position(base, offsets[1]), make_position(base, offsets[2]), make_position(base, offsets[3])]
}

pub(crate) fn make_rectangle_by_corners(top_left_x: usize, top_left_y: usize, bottom_right_x: usize, bottom_right_y: usize) -> Rect<isize, Pixel> {
    let origin = Point2D::new(top_left_x as isize, top_left_y as isize);
    let size = Size2D::new(bottom_right_x as isize - top_left_x as isize, bottom_right_y as isize - top_left_y as isize);
    Rect::new(origin, size)
}
