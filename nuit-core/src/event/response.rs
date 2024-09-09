use serde::{Deserialize, Serialize};

use crate::Node;

/// A UI event response.
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
