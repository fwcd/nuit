use crate::ShapeNode;

use super::Shape;

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
