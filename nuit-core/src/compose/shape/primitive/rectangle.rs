use crate::{Shape, ShapeNode};

/// A rectangular shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle;

impl Rectangle {
    pub fn new() -> Self {
        Self
    }
}

impl Shape for Rectangle {
    fn render(&self) -> ShapeNode {
        ShapeNode::Rectangle {}
    }
}
