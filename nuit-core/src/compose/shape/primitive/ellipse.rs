use crate::{Shape, ShapeNode};

/// An elliptic shape.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Ellipse;

impl Ellipse {
    pub const fn new() -> Self {
        Self
    }
}

impl Shape for Ellipse {
    fn render(&self) -> ShapeNode {
        ShapeNode::Ellipse {}
    }
}
