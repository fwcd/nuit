use serde::{Serialize, Deserialize};

use crate::Insets;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Modifier {
    Padding { insets: Insets },
}
