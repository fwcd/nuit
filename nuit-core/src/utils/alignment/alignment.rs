use serde::{Serialize, Deserialize};

use super::{HorizontalAlignment, VerticalAlignment};

/// A 2D alignment.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alignment {
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
}

impl Alignment {
    pub const TOP_LEADING: Self = Self::new(HorizontalAlignment::Leading, VerticalAlignment::Top);
    pub const TOP: Self = Self::new(HorizontalAlignment::Center, VerticalAlignment::Top);
    pub const TOP_TRAILING: Self = Self::new(HorizontalAlignment::Trailing, VerticalAlignment::Top);

    pub const LEADING: Self = Self::with_horizontal(HorizontalAlignment::Leading);
    pub const CENTER: Self = Self::with_horizontal(HorizontalAlignment::Center);
    pub const TRAILING: Self = Self::with_horizontal(HorizontalAlignment::Trailing);

    pub const BOTTOM_LEADING: Self = Self::new(HorizontalAlignment::Leading, VerticalAlignment::Bottom);
    pub const BOTTOM: Self = Self::new(HorizontalAlignment::Center, VerticalAlignment::Bottom);
    pub const BOTTOM_TRAILING: Self = Self::new(HorizontalAlignment::Trailing, VerticalAlignment::Bottom);

    /// Creates a new alignment from the given horizontal and vertical alignments.
    pub const fn new(horizontal: HorizontalAlignment, vertical: VerticalAlignment) -> Self {
        Self { horizontal, vertical }
    }

    /// Creates a new, vertically centered alignment from the given horizontal alignment.
    pub const fn with_horizontal(horizontal: HorizontalAlignment) -> Self {
        Self { horizontal, vertical: VerticalAlignment::Center }
    }

    /// Creates a new, horizontally centered alignment from the given vertical alignment.
    pub const fn with_vertical(vertical: VerticalAlignment) -> Self {
        Self { horizontal: HorizontalAlignment::Center, vertical }
    }

    /// The horizontal component of this alignment.
    pub const fn horizontal(self) -> HorizontalAlignment {
        self.horizontal
    }

    /// The vertical component of this alignment.
    pub const fn vertical(self) -> VerticalAlignment {
        self.vertical
    }
}
