use serde::{Deserialize, Serialize};

use super::DragEvent;

/// A gesture-related UI event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GestureEvent {
    Tap {},
    Drag { drag: DragEvent },
}
