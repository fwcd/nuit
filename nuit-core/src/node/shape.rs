use serde::{Serialize, Deserialize};

use crate::{Angle, Style, Vec2};

/// A rendered shape tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShapeNode {
    // Primitive
    Capsule {},
    Circle {},
    Ellipse {},
    Rectangle {},
    RoundedRectangle { corner_size: Vec2<f64> },
    Sector { start_angle: Angle, end_angle: Angle, inner_radius_fraction: f64 },

    // Styled
    Fill { wrapped: Box<ShapeNode>, style: Style },
    Stroke { wrapped: Box<ShapeNode>, style: Style },
}
