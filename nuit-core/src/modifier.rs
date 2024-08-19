use serde::{Serialize, Deserialize};

use crate::{Frame, Insets, Style, Vec2};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Modifier {
    Padding { insets: Insets },
    Position { position: Vec2<f64> },
    Frame { frame: Frame },
    Fill { style: Style },
}
