use serde::{Serialize, Deserialize};

use crate::Color;

use super::{BlendMode, Material, SemanticStyle, Shadow};

/// A color or pattern for filling or stroking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Style {
    // Primitive
    Color { color: Color },
    Hierarchical { level: usize },
    Material { material: Material },
    Semantic { style: SemanticStyle },

    // Modifiers
    BlendMode { wrapped: Box<Style>, blend_mode: BlendMode },
    Opacity { wrapped: Box<Style>, opacity: f64 },
    Shadow { wrapped: Box<Style>, shadow: Shadow },
}

impl Style {
    pub const PRIMARY: Self = Self::hierarchical(0);
    pub const SECONDARY: Self = Self::hierarchical(1);
    pub const TERTIARY: Self = Self::hierarchical(2);
    pub const QUATERNARY: Self = Self::hierarchical(3);
    pub const QUINARY: Self = Self::hierarchical(4);

    pub const FOREGROUND: Self = Self::semantic(SemanticStyle::Foreground);
    pub const BACKGROUND: Self = Self::semantic(SemanticStyle::Background);
    pub const SELECTION: Self = Self::semantic(SemanticStyle::Selection);
    pub const SEPARATOR: Self = Self::semantic(SemanticStyle::Separator);
    pub const TINT: Self = Self::semantic(SemanticStyle::Tint);
    pub const PLACEHOLDER: Self = Self::semantic(SemanticStyle::Placeholder);
    pub const LINK: Self = Self::semantic(SemanticStyle::Link);
    pub const FILL: Self = Self::semantic(SemanticStyle::Fill);
    pub const WINDOW_BACKGROUND: Self = Self::semantic(SemanticStyle::WindowBackground);

    #[must_use]
    pub const fn color(color: Color) -> Self {
        Self::Color { color }
    }

    #[must_use]
    pub const fn hierarchical(level: usize) -> Self {
        Self::Hierarchical { level }
    }

    #[must_use]
    pub const fn material(material: Material) -> Self {
        Self::Material { material }
    }

    #[must_use]
    pub const fn semantic(style: SemanticStyle) -> Self {
        Self::Semantic { style }
    }

    #[must_use]
    pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
        Self::BlendMode { wrapped: Box::new(self), blend_mode }
    }

    #[must_use]
    pub fn opacity(self, opacity: f64) -> Self {
        Self::Opacity { wrapped: Box::new(self), opacity }
    }

    #[must_use]
    pub fn shadow(self, shadow: Shadow) -> Self {
        Self::Shadow { wrapped: Box::new(self), shadow }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::Color { color }
    }
}

impl From<Material> for Style {
    fn from(material: Material) -> Self {
        Self::Material { material }
    }
}

impl From<SemanticStyle> for Style {
    fn from(style: SemanticStyle) -> Self {
        Self::Semantic { style }
    }
}
