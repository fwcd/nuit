use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{Geometry, Id};

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
    ToggleChange {},

    // Navigation
    UpdateNavigationPath { path: Vec<Value> },
    GetNavigationDestination { value: Value },

    // Layout
    GetGeometryReaderView { geometry: Geometry },

    // Lifecycle
    Appear,
    Disappear,
}
