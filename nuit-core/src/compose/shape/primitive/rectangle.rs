use crate::{Shape, ShapeNode};

/// A rectangular shape.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rectangle;

impl Rectangle {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Shape for Rectangle {
    fn render(&self) -> ShapeNode {
        ShapeNode::Rectangle {}
    }
}
