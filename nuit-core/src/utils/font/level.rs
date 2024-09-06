use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FontLevel {
    ExtraLargeTitle2,
    ExtraLargeTitle,
    LargeTitle,
    Title,
    Title2,
    Title3,
    Headline,
    Subheadline,
    #[default]
    Body,
    Callout,
    Caption,
    Caption2,
    Footnote,
}
