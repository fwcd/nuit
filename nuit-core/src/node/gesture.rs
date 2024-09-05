use serde::{Deserialize, Serialize};

/// A rendered gesture tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GestureNode {
    Tap { count: usize },
    Drag { minimum_distance: f64 },
}
