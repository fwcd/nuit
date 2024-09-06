use serde::{Serialize, Deserialize};

/// An alignment along the x-axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    Leading,
    #[default]
    Center,
    Trailing,
}
