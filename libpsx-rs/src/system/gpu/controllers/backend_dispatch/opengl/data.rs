use crate::system::gpu::types::{
    rendering::ClutKind,
    rendering::TransparencyKind,
};

pub(crate) fn transparency_mode_value(transparency: TransparencyKind) -> u32 {
    match transparency {
        TransparencyKind::Opaque => 0,
        TransparencyKind::Average => 1,
        TransparencyKind::Additive => 2,
        TransparencyKind::Difference => 3,
        TransparencyKind::Quarter => 4,
    }
}

pub(crate) fn clut_mode_value(clut_kind: ClutKind) -> u32 {
    match clut_kind {
        ClutKind::Bits4 {
            ..
        } => 0,
        ClutKind::Bits8 {
            ..
        } => 1,
        ClutKind::Direct => 2,
    }
}

pub(crate) fn clut_base_value(clut_kind: ClutKind) -> [f32; 2] {
    match clut_kind {
        ClutKind::Bits4 {
            base,
        } => [base.x, base.y],
        ClutKind::Bits8 {
            base,
        } => [base.x, base.y],
        ClutKind::Direct => [0.0, 0.0],
    }
}
