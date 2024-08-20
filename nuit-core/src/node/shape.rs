use serde::{Serialize, Deserialize};

use crate::{Style, Vec2};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShapeNode {
    // Primitive
    Capsule {},
    Circle {},
    Ellipse {},
    Rectangle {},
    RoundedRectangle { corner_size: Vec2<f64> },

    // Styled
    Fill { wrapped: Box<ShapeNode>, style: Style },
    Stroke { wrapped: Box<ShapeNode>, style: Style },
}
