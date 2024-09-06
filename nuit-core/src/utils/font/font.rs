use serde::{Deserialize, Serialize};

use super::{FontDesign, FontLevel, FontSize, FontWeight};

/// An abstract/platform-dependent font.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Font {
    System { size: FontSize, design: Option<FontDesign>, weight: Option<FontWeight> },
    Custom { name: String, size: f64 },
}

impl Font {
    pub const EXTRA_LARGE_TITLE2: Self = Self::with_level(FontLevel::ExtraLargeTitle2);
    pub const EXTRA_LARGE_TITLE: Self = Self::with_level(FontLevel::ExtraLargeTitle);
    pub const LARGE_TITLE: Self = Self::with_level(FontLevel::LargeTitle);
    pub const TITLE: Self = Self::with_level(FontLevel::Title);
    pub const TITLE2: Self = Self::with_level(FontLevel::Title2);
    pub const TITLE3: Self = Self::with_level(FontLevel::Title3);
    pub const HEADLINE: Self = Self::with_level(FontLevel::Headline);
    pub const SUBHEADLINE: Self = Self::with_level(FontLevel::Subheadline);
    pub const BODY: Self = Self::with_level(FontLevel::Body);
    pub const CALLOUT: Self = Self::with_level(FontLevel::Callout);
    pub const CAPTION: Self = Self::with_level(FontLevel::Caption);
    pub const CAPTION2: Self = Self::with_level(FontLevel::Caption2);
    pub const FOOTNOTE: Self = Self::with_level(FontLevel::Footnote);

    pub fn system(size: impl Into<FontSize>, design: Option<FontDesign>, weight: Option<FontWeight>) -> Self {
        Self::System { size: size.into(), design, weight }
    }

    pub fn custom(name: impl Into<String>, size: impl Into<f64>) -> Self {
        Self::Custom { name: name.into(), size: size.into() }
    }

    pub fn with_size(size: impl Into<FontSize>) -> Self {
        Self::System { size: size.into(), design: None, weight: None }
    }

    pub const fn with_level(level: FontLevel) -> Self {
        Self::System { size: FontSize::level(level), design: None, weight: None }
    }
}
