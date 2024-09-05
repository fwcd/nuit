use serde::{Deserialize, Serialize};

/// A gesture-related UI event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GestureEvent {
    Tap {},
}
