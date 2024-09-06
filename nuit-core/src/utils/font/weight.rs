use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FontWeight {
    Black,
    Bold,
    Heavy,
    Light,
    Medium,
    #[default]
    Regular,
    Semibold,
    Thin,
    UltraLight,
}
