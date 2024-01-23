use serde::{Serialize, Deserialize};

use crate::Id;

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
