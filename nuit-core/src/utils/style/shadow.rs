use serde::{Deserialize, Serialize};

use crate::{Color, Vec2};

/// A style for rendered shadows.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow {
    kind: ShadowKind,
    color: Color,
    radius: f64,
    offset: Vec2<f64>,
}

/// A type of shadow.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShadowKind {
    #[default]
    Drop,
    Inner,
}

impl Shadow {
    pub const fn new(kind: ShadowKind, color: Color, radius: f64, offset: Vec2<f64>) -> Self {
        Self { kind, color, radius, offset }
    }

    pub const fn drop(color: Color, radius: f64, offset: Vec2<f64>) -> Self {
        Self::new(ShadowKind::Drop, color, radius, offset)
    }

    pub const fn inner(color: Color, radius: f64, offset: Vec2<f64>) -> Self {
        Self::new(ShadowKind::Inner, color, radius, offset)
    }
}
