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
