use serde::{Serialize, Deserialize};

use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Style {
    Color { color: Color },
    Hierarchical { level: usize },
}

impl Style {
    pub const fn color(color: Color) -> Self {
        Self::Color { color }
    }

    pub const fn hierarchical(level: usize) -> Self {
        Self::Hierarchical { level }
    }
}
