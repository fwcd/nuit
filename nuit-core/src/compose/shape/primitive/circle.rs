use crate::{Shape, ShapeNode};

/// A circular shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle;

impl Circle {
    pub fn new() -> Self {
        Self
    }
}

impl Shape for Circle {
    fn render(&self) -> ShapeNode {
        ShapeNode::Circle {}
    }
}
