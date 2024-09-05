use serde::{Deserialize, Serialize};

use crate::Vec2;

/// A drag event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragEvent {
    start_location: Vec2<f64>,
    location: Vec2<f64>,
}
