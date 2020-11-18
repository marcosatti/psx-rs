use crate::{
    system::{
        gpu::types::{
            rendering::ClutKind,
            TransparencyMode,
        },
    },
};

pub(crate) fn transparency_value(transparency: TransparencyMode) -> u32 {
    match transparency {
        TransparencyMode::Average => 0,
        TransparencyMode::Additive => 1,
        TransparencyMode::Difference => 2,
        TransparencyMode::Quarter => 3,
    }
}

pub(crate) fn clut_mode_value(clut_kind: ClutKind) -> u32 {
    match clut_kind {
        ClutKind::Bits4 { .. } => 0,
        ClutKind::Bits8 { .. } => 1,
        ClutKind::Direct => 2,
    }
}

pub(crate) fn clut_base_value(clut_kind: ClutKind) -> [f32; 2] {
    match clut_kind {
        ClutKind::Bits4 { base } => [base.x, base.y],
        ClutKind::Bits8 { base } => [base.x, base.y],
        ClutKind::Direct => [0.0, 0.0],
    }
}
