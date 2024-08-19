use serde::{Serialize, Deserialize};

use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Style {
    Color { color: Color },
    Hierarchical { level: usize },
}

impl Style {
    pub const PRIMARY: Self = Self::hierarchical(0);
    pub const SECONDARY: Self = Self::hierarchical(1);
    pub const TERTIARY: Self = Self::hierarchical(2);
    pub const QUATERNARY: Self = Self::hierarchical(3);
    pub const QUINARY: Self = Self::hierarchical(4);

    pub const fn color(color: Color) -> Self {
        Self::Color { color }
    }

    pub const fn hierarchical(level: usize) -> Self {
        Self::Hierarchical { level }
    }
}
