use serde::{Deserialize, Serialize};

use crate::Node;

/// A response to a UI event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum EventResponse {
    Empty {},
    Node { node: Node },
}

impl Default for EventResponse {
    fn default() -> Self {
        Self::Empty {}
    }
}
