use crate::{Angle, Shape, ShapeNode};

/// A circular sector shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Sector {
    start_angle: Angle,
    end_angle: Angle,
    inner_radius: f64,
    outer_radius: f64,
}

impl Sector {
    pub fn new(
        start_angle: impl Into<Angle>,
        end_angle: impl Into<Angle>,
        inner_radius: impl Into<f64>,
        outer_radius: impl Into<f64>,
    ) -> Self {
        Self {
            start_angle: start_angle.into(),
            end_angle: end_angle.into(),
            inner_radius: inner_radius.into(),
            outer_radius: outer_radius.into(),
        }
    }
}

impl Shape for Sector {
    fn render(&self) -> ShapeNode {
        ShapeNode::Sector {
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            inner_radius: self.inner_radius,
            outer_radius: self.outer_radius,
        }
    }
}
