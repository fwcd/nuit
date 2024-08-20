use crate::{Shape, ShapeNode, Vec2};

/// A rectangular shape with rounded corners.
#[derive(Debug, Clone, PartialEq)]
pub struct RoundedRectangle {
    corner_size: Vec2<f64>,
}

impl RoundedRectangle {
    pub fn new(corner_size: Vec2<f64>) -> Self {
        Self { corner_size }
    }

    pub fn with_corner_radius(corner_radius: f64) -> Self {
        Self::new(Vec2::new(corner_radius, corner_radius))
    }
}

impl Shape for RoundedRectangle {
    fn render(&self) -> ShapeNode {
        ShapeNode::RoundedRectangle {
            corner_size: self.corner_size
        }
    }
}
