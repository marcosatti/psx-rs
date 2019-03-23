use crate::constants::gpu::*;
use crate::types::color::Color;
use crate::types::bitfield::Bitfield;
use crate::types::geometry::*;

#[derive(Copy, Clone, Debug)]
enum TransparencyMode {
    Average,
    Additive,
    Difference,
    Quarter,
}

#[derive(Copy, Clone, Debug)]
enum ClutMode {
    Bits4,
    Bits8,
    Bits15,
    Reserved,
}

pub fn extract_texpage_transparency_mode(texpage_raw: u32) -> TransparencyMode {
    match Bitfield::new(5, 2).extract_from(texpage_raw) {
        0 => TransparencyMode::Average,
        1 => TransparencyMode::Additive,
        2 => TransparencyMode::Difference,
        3 => TransparencyMode::Quarter,
        _ => unreachable!("Invalid transparency mode"),
    }
}

pub fn extract_texpage_clut_mode(texpage_raw: u32) -> ClutMode {
    match Bitfield::new(7, 2).extract_from(texpage_raw) {
        0 => ClutMode::Bits4,
        1 => ClutMode::Bits8,
        2 => ClutMode::Bits15,
        3 => ClutMode::Reserved,
        _ => unreachable!("Invalid CLUT mode"),
    }
}

pub fn extract_color_rgb(color_raw: u32, alpha: u8) -> Color {
    Color {
        r: Bitfield::new(0, 8).extract_from(color_raw) as u8, 
        g: Bitfield::new(8, 8).extract_from(color_raw) as u8, 
        b: Bitfield::new(16, 8).extract_from(color_raw) as u8, 
        a: alpha,
    }
}

pub fn extract_colors_3_rgb(colors_raw: [u32; 3], alpha: u8) -> [Color; 3] {
    [
        extract_color_rgb(colors_raw[0], alpha),
        extract_color_rgb(colors_raw[1], alpha),
        extract_color_rgb(colors_raw[2], alpha),
    ]
}

pub fn extract_colors_4_rgb(colors_raw: [u32; 4], alpha: u8) -> [Color; 4] {
    [
        extract_color_rgb(colors_raw[0], alpha),
        extract_color_rgb(colors_raw[1], alpha),
        extract_color_rgb(colors_raw[2], alpha),
        extract_color_rgb(colors_raw[3], alpha),
    ]
}

pub fn normalize_point(point: Point2D<usize, Pixel>) -> Point2D<f64, Normalized> {
    Point2D::new(
        (point.x as f64 - ((VRAM_WIDTH_16B as f64 / 2.0) - 1.0)) / ((VRAM_WIDTH_16B as f64 / 2.0) - 1.0),
        -((point.y as f64 - ((VRAM_HEIGHT_LINES as f64 / 2.0) - 1.0)) / ((VRAM_HEIGHT_LINES as f64 / 2.0) - 1.0)),
    )
}

pub fn extract_point(point_raw: u32) -> Point2D<usize, Pixel> {
    Point2D::new(
        Bitfield::new(0, 16).extract_from(point_raw) as usize,
        Bitfield::new(16, 16).extract_from(point_raw) as usize,
    )
}

pub fn extract_point_normalized(point_raw: u32) -> Point2D<f64, Normalized> {
    normalize_point(extract_point(point_raw))
}

pub fn normalize_size(size: Size2D<usize, Pixel>) -> Size2D<f64, Normalized> {
    Size2D::new(
        (size.width as f64 / VRAM_WIDTH_16B as f64) * 2.0, 
        (size.height as f64 / VRAM_HEIGHT_LINES as f64) * 2.0, 
    )
}

pub fn extract_size(size_raw: u32) -> Size2D<usize, Pixel> {
    Size2D::new(
        Bitfield::new(0, 16).extract_from(size_raw) as usize, 
        Bitfield::new(16, 16).extract_from(size_raw) as usize,
    )
}

pub fn extract_size_normalized(size_raw: u32) -> Size2D<f64, Normalized> {
    normalize_size(extract_size(size_raw))
}

pub fn normalize_points_3(points: [Point2D<usize, Pixel>; 3]) -> [Point2D<f64, Normalized>; 3] {
    [
        normalize_point(points[0]),
        normalize_point(points[1]),
        normalize_point(points[2]),
    ]
}

pub fn extract_vertices_3(vertices_raw: [u32; 3]) -> [Point2D<usize, Pixel>; 3] {
    [
        extract_point(vertices_raw[0]),
        extract_point(vertices_raw[1]),
        extract_point(vertices_raw[2]),
    ]
}

pub fn extract_vertices_3_normalized(vertices_raw: [u32; 3]) -> [Point2D<f64, Normalized>; 3] {
    normalize_points_3(extract_vertices_3(vertices_raw))
}

pub fn normalize_points_4(points: [Point2D<usize, Pixel>; 4]) -> [Point2D<f64, Normalized>; 4] {
    [
        normalize_point(points[0]),
        normalize_point(points[1]),
        normalize_point(points[2]),
        normalize_point(points[3]),
    ]
}

pub fn extract_vertices_4(vertices_raw: [u32; 4]) -> [Point2D<usize, Pixel>; 4] {
    [
        extract_point(vertices_raw[0]),
        extract_point(vertices_raw[1]),
        extract_point(vertices_raw[2]),
        extract_point(vertices_raw[3]),
    ]
}

pub fn extract_vertices_4_normalized(vertices_raw: [u32; 4]) -> [Point2D<f64, Normalized>; 4] {
    normalize_points_4(extract_vertices_4(vertices_raw))
}

pub fn extract_texcoords_4(texpage_raw: u32, clut_mode: ClutMode, texcoords_raw: [u32; 4]) -> [Point2D<usize, Pixel>; 4] {
    let texcoord_x_bitfield = Bitfield::new(0, 8);
    let texcoord_y_bitfield = Bitfield::new(8, 8);

    // The texcoords are in terms of texture pixels, not framebuffer pixels (see clut_mode below).
    let mut texcoord_offset_points: [Point2D<usize, Pixel>; 4] = [
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[0]) as usize, texcoord_y_bitfield.extract_from(texcoords_raw[0]) as usize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[1]) as usize, texcoord_y_bitfield.extract_from(texcoords_raw[1]) as usize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[2]) as usize, texcoord_y_bitfield.extract_from(texcoords_raw[2]) as usize),
        Point2D::new(texcoord_x_bitfield.extract_from(texcoords_raw[3]) as usize, texcoord_y_bitfield.extract_from(texcoords_raw[3]) as usize),
    ];

    for i in 0..4 {
        // Each framebuffer pixel represents {scale_factor} number of texture pixels.
        let scale_factor = match clut_mode {
            Bits4 => 4,
            _ => unimplemented!("Extracting texcoords CLUT mode unimplemented: {:?}", clut_mode),
        };

        if texcoord_offset_points[i].x % scale_factor != 0 { panic!("Unhandled case where texcoord offset is not fully divisible by scale factor"); }
        texcoord_offset_points[i].x = texcoord_offset_points[i].x / scale_factor;
        if texcoord_offset_points[i].y % scale_factor != 0 { panic!("Unhandled case where texcoord offset is not fully divisible by scale factor"); }
        texcoord_offset_points[i].y = texcoord_offset_points[i].y / scale_factor;
    }

    let texpage = Bitfield::new(16, 16).extract_from(texpage_raw);
    let texpage_x_base = (Bitfield::new(0, 4).extract_from(texpage) * 64) as usize;
    let texpage_y_base = (Bitfield::new(4, 1).extract_from(texpage) * 256) as usize;

    let mut texcoords: [Point2D<usize, Pixel>; 4] = [Point2D::new(texpage_x_base, texpage_y_base); 4];

    for i in 0..4 {
        texcoords[i].x += texcoord_offset_points[i].x;
        texcoords[i].y += texcoord_offset_points[i].y;
    }

    texcoords
}

pub fn extract_texcoords_4_normalized(texpage_raw: u32, clut_mode: ClutMode, texcoords_raw: [u32; 4]) -> [Point2D<f64, Normalized>; 4] {
    normalize_points_4(extract_texcoords(texpage_raw, clut_mode, texcoords_raw))
}

pub fn extract_clut_base(clut_raw: u32) -> Point2D<usize, Pixel> {
    let clut = Bitfield::new(16, 16).extract_from(clut_raw);
    let clut_x = (Bitfield::new(0, 6).extract_from(clut) * 16) as usize;
    let clut_y = Bitfield::new(6, 9).extract_from(clut) as usize;

    Point2D::new(
        clut_x, 
        clut_y,
    )
}

pub fn extract_clut_base_normalized(clut_raw: u32) -> Point2D<f64, Normalized> {
    normalize_point(extract_clut_base(clut_raw))
}
