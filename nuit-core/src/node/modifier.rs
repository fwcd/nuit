use serde::{Serialize, Deserialize};

use crate::{Alignment, Angle, Font, Frame, Insets, Style, UnitPoint, Vec2};

/// A rendered modifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ModifierNode {
    Padding { insets: Insets },
    Position { position: Vec2<f64> },
    Offset { delta: Vec2<f64> },
    Opacity { opacity: f64 },
    Frame { frame: Frame, alignment: Alignment },
    Fill { style: Style },
    Font { font: Font },
    ForegroundStyle { style: Style },
    ScaleEffect { factor: f64, anchor: UnitPoint },
    RotationEffect { angle: Angle, anchor: UnitPoint },
    Help { text: String },
}
