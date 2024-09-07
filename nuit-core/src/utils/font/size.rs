use serde::{Deserialize, Serialize};

use super::FontLevel;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FontSize {
    Level { level: FontLevel },
    Custom { size: f64 },
}

impl FontSize {
    pub const EXTRA_LARGE_TITLE2: Self = Self::level(FontLevel::ExtraLargeTitle2);
    pub const EXTRA_LARGE_TITLE: Self = Self::level(FontLevel::ExtraLargeTitle);
    pub const LARGE_TITLE: Self = Self::level(FontLevel::LargeTitle);
    pub const TITLE: Self = Self::level(FontLevel::Title);
    pub const TITLE2: Self = Self::level(FontLevel::Title2);
    pub const TITLE3: Self = Self::level(FontLevel::Title3);
    pub const HEADLINE: Self = Self::level(FontLevel::Headline);
    pub const SUBHEADLINE: Self = Self::level(FontLevel::Subheadline);
    pub const BODY: Self = Self::level(FontLevel::Body);
    pub const CALLOUT: Self = Self::level(FontLevel::Callout);
    pub const CAPTION: Self = Self::level(FontLevel::Caption);
    pub const CAPTION2: Self = Self::level(FontLevel::Caption2);
    pub const FOOTNOTE: Self = Self::level(FontLevel::Footnote);

    #[must_use]
    pub const fn level(level: FontLevel) -> Self {
        Self::Level { level }
    }

    #[must_use]
    pub const fn custom(size: f64) -> Self {
        Self::Custom { size }
    }
}

impl From<usize> for FontSize {
    #[allow(clippy::cast_precision_loss)]
    fn from(size: usize) -> Self {
        Self::Custom { size: size as f64 }
    }
}

impl From<f64> for FontSize {
    fn from(size: f64) -> Self {
        Self::Custom { size }
    }
}

impl From<FontLevel> for FontSize {
    fn from(level: FontLevel) -> Self {
        Self::Level { level }
    }
}
