use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::Id;

use super::GestureEvent;

/// A UI event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Event {
    // Interaction
    ButtonTap {},
    Gesture { gesture: GestureEvent },
    UpdateText { content: String },
    UpdatePickerSelection { id: Id },
    UpdateSliderValue { value: f64 },

    // Navigation
    UpdateNavigationPath { path: Vec<Value> },

    // Lifecycle
    Appear,
    Disappear,
}
