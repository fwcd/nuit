use crate::{Angle, Shape, ShapeNode};

/// A circular sector shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Sector {
    start_angle: Angle,
    end_angle: Angle,
    outer_radius: f64,
    inner_radius: f64,
}

impl Sector {
    pub fn new(
        start_angle: impl Into<Angle>,
        end_angle: impl Into<Angle>,
        outer_radius: impl Into<f64>,
        inner_radius: impl Into<f64>,
    ) -> Self {
        Self {
            start_angle: start_angle.into(),
            end_angle: end_angle.into(),
            outer_radius: outer_radius.into(),
            inner_radius: inner_radius.into(),
        }
    }
}

impl Shape for Sector {
    fn render(&self) -> ShapeNode {
        ShapeNode::Sector {
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            outer_radius: self.outer_radius,
            inner_radius: self.inner_radius,
        }
    }
}
