use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Animation {
    Default {},
}

impl Default for Animation {
    fn default() -> Self {
        Self::Default {}
    }
}
