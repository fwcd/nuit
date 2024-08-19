use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Material {
    UltraThin,
    Thin,
    Regular,
    Thick,
    UltraThick,
    Bar,
}
