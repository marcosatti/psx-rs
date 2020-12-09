use crate::{
    system::gpu::types::{
        ClutMode,
        TransparencyMode,
    },
    types::{
        color::*,
        geometry::*,
    },
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum RenderingKind {
    Shaded,
    TextureBlending {
        page_base: Point2D<usize, Pixel>,
        clut_kind: ClutKind,
    },
    RawTexture {
        page_base: Point2D<usize, Pixel>,
        clut_kind: ClutKind,
    },
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ClutKind {
    Direct,
    Bits4 {
        clut_base: Point2D<usize, Pixel>,
    },
    Bits8 {
        clut_base: Point2D<usize, Pixel>,
    },
}

impl ClutKind {
    pub(crate) fn from_data(mode: ClutMode, base: Point2D<usize, Pixel>) -> ClutKind {
        match mode {
            ClutMode::Bits15 => ClutKind::Direct,
            ClutMode::Bits4 => {
                ClutKind::Bits4 {
                    clut_base: base,
                }
            },
            ClutMode::Bits8 => {
                ClutKind::Bits8 {
                    clut_base: base,
                }
            },
            ClutMode::Reserved => unreachable!("Reserved CLUT mode used!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum TransparencyKind {
    Opaque,
    Average,
    Additive,
    Difference,
    Quarter,
}

impl TransparencyKind {
    pub(crate) fn from_data(mode: TransparencyMode) -> TransparencyKind {
        match mode {
            TransparencyMode::Additive => TransparencyKind::Additive,
            TransparencyMode::Average => TransparencyKind::Average,
            TransparencyMode::Difference => TransparencyKind::Difference,
            TransparencyMode::Quarter => TransparencyKind::Quarter,
        }
    }
}

pub(crate) struct ReadFramebufferParams {
    pub(crate) rectangle: Rect<usize, Pixel>,
}

pub(crate) struct WriteFramebufferParams<'a> {
    pub(crate) rectangle: Rect<usize, Pixel>,
    pub(crate) data: &'a [PackedColor],
    pub(crate) mask_bit_force_set: bool,
    pub(crate) mask_bit_check: bool,
}

pub(crate) struct RectangleParams {
    pub(crate) rectangle: Rect<isize, Pixel>,
    pub(crate) color: Color,
    pub(crate) texture_position_base_offset: Size2D<isize, Pixel>,
    pub(crate) rendering_kind: RenderingKind,
    pub(crate) transparency_kind: TransparencyKind,
    pub(crate) drawing_area: Rect<isize, Pixel>,
    pub(crate) mask_bit_force_set: bool,
    pub(crate) mask_bit_check: bool,
}

pub(crate) struct TrianglesParams<'a> {
    pub(crate) vertices: usize,
    pub(crate) positions: &'a [Point2D<isize, Pixel>],
    pub(crate) colors: &'a [Color],
    pub(crate) texture_position_offsets: &'a [Size2D<isize, Pixel>],
    pub(crate) rendering_kind: RenderingKind,
    pub(crate) transparency_kind: TransparencyKind,
    pub(crate) drawing_area: Rect<isize, Pixel>,
    pub(crate) mask_bit_force_set: bool,
    pub(crate) mask_bit_check: bool,
}
