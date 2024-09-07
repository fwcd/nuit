use crate::{Shape, ShapeNode};

/// A circular shape.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Circle;

impl Circle {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Shape for Circle {
    fn render(&self) -> ShapeNode {
        ShapeNode::Circle {}
    }
}
