use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SemanticStyle {
    Foreground,
    Background,
    Selection,
    Separator,
    Tint,
    Placeholder,
    Link,
    Fill,
    WindowBackground,
}
