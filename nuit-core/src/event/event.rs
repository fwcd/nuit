use serde::{Serialize, Deserialize};

use crate::Id;

use super::GestureEvent;

/// A UI event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Event {
    // Interaction
    Click {},
    Gesture { gesture: GestureEvent },
    UpdateText { content: String },
    UpdatePickerSelection { id: Id },

    // Lifecycle
    Appear,
    Disappear,
}
