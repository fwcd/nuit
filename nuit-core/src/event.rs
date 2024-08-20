use serde::{Serialize, Deserialize};

use crate::Id;

/// A UI event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    // Interaction
    Click {},
    UpdateText { content: String },
    UpdatePickerSelection { id: Id },

    // Lifecycle
    Appear,
    Disappear,
}
