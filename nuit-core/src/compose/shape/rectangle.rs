use crate::ShapeNode;

use super::Shape;

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
