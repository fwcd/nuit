use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    // Interaction
    Click {},
    UpdateText { content: String },

    // Lifecycle
    Appear,
    Disappear,
}
