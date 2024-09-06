use serde::{Serialize, Deserialize};

/// An alignment along the y-axis.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerticalAlignment {
    Top,
    #[default]
    Center,
    Bottom,
    FirstTextBaseline,
    LastTextBaseline,
}
