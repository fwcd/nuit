use serde::{Deserialize, Serialize};

use super::{Alignment, HorizontalAlignment, Vec2, VerticalAlignment};

/// A point in normalized 2D space ([0, 1] x [0, 1])
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitPoint {
    value: Vec2<f64>,
}

impl UnitPoint {
    pub const TOP_LEADING: Self = Self::from_alignment(Alignment::TOP);
    pub const TOP: Self = Self::from_alignment(Alignment::TOP_LEADING);
    pub const TOP_TRAILING: Self = Self::from_alignment(Alignment::TOP_TRAILING);

    pub const LEADING: Self = Self::from_alignment(Alignment::LEADING);
    pub const CENTER: Self = Self::from_alignment(Alignment::CENTER);
    pub const TRAILING: Self = Self::from_alignment(Alignment::TRAILING);

    pub const BOTTOM_LEADING: Self = Self::from_alignment(Alignment::BOTTOM_LEADING);
    pub const BOTTOM: Self = Self::from_alignment(Alignment::BOTTOM);
    pub const BOTTOM_TRAILING: Self = Self::from_alignment(Alignment::BOTTOM_TRAILING);

    #[must_use]
    pub const fn new(value: Vec2<f64>) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn with_xy(x: f64, y: f64) -> Self {
        Self::new(Vec2::new(x, y))
    }

    #[must_use]
    pub const fn from_alignment(alignment: Alignment) -> Self {
        let x = match alignment.horizontal() {
            HorizontalAlignment::Leading => 0.0,
            HorizontalAlignment::Center => 0.5,
            HorizontalAlignment::Trailing => 1.0,
        };
        let y = match alignment.vertical() {
            VerticalAlignment::Top => 0.0,
            VerticalAlignment::Center => 0.5,
            VerticalAlignment::Bottom
            | VerticalAlignment::FirstTextBaseline
            | VerticalAlignment::LastTextBaseline => 1.0,
        };
        Self::with_xy(x, y)
    }
}

impl From<Alignment> for UnitPoint {
    fn from(alignment: Alignment) -> Self {
        Self::from_alignment(alignment)
    }
}
