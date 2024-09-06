use serde::{Deserialize, Serialize};

use super::{Alignment, Vec2};

/// A point in normalized 2D space ([0, 1] x [0, 1])
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitPoint {
    value: Vec2<f64>,
}

impl UnitPoint {
    pub const TOP_LEADING: Self = Self::with_xy(0.0, 0.0);
    pub const TOP: Self = Self::with_xy(0.5, 0.0);
    pub const TOP_TRAILING: Self = Self::with_xy(1.0, 0.0);

    pub const LEADING: Self = Self::with_xy(0.0, 0.5);
    pub const CENTER: Self = Self::with_xy(0.5, 0.5);
    pub const TRAILING: Self = Self::with_xy(1.0, 0.5);

    pub const BOTTOM_LEADING: Self = Self::with_xy(0.0, 0.5);
    pub const BOTTOM: Self = Self::with_xy(0.5, 0.5);
    pub const BOTTOM_TRAILING: Self = Self::with_xy(1.0, 0.5);

    pub const fn new(value: Vec2<f64>) -> Self {
        Self { value }
    }

    pub const fn with_xy(x: f64, y: f64) -> Self {
        Self::new(Vec2::new(x, y))
    }
}

impl From<Alignment> for UnitPoint {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::TopLeading => Self::TOP_LEADING,
            Alignment::Top => Self::TOP,
            Alignment::TopTrailing => Self::TOP_TRAILING,
            Alignment::Leading => Self::LEADING,
            Alignment::Center => Self::CENTER,
            Alignment::Trailing => Self::TRAILING,
            Alignment::BottomLeading => Self::BOTTOM_LEADING,
            Alignment::Bottom => Self::BOTTOM,
            Alignment::BottomTrailing => Self::BOTTOM_TRAILING,
        }
    }
}
