use serde::{Serialize, Deserialize};

use crate::Vec2;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShapeNode {
    Capsule {},
    Circle {},
    Ellipse {},
    Rectangle {},
    RoundedRectangle { corner_size: Vec2<f64> },
}
