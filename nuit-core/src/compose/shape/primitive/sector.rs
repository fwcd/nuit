use crate::{Angle, Shape, ShapeNode};

/// A circular sector shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Sector {
    start_angle: Angle,
    end_angle: Angle,
    inner_radius_fraction: f64,
}

impl Sector {
    pub fn new(
        start_angle: impl Into<Angle>,
        end_angle: impl Into<Angle>,
        inner_radius_fraction: impl Into<f64>,
    ) -> Self {
        Self {
            start_angle: start_angle.into(),
            end_angle: end_angle.into(),
            inner_radius_fraction: inner_radius_fraction.into(),
        }
    }
}

impl Shape for Sector {
    fn render(&self) -> ShapeNode {
        ShapeNode::Sector {
            start_angle: self.start_angle,
            end_angle: self.end_angle,
            inner_radius_fraction: self.inner_radius_fraction,
        }
    }
}
