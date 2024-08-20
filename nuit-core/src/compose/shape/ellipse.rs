use crate::ShapeNode;

use super::Shape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ellipse;

impl Ellipse {
    pub fn new() -> Self {
        Self
    }
}

impl Shape for Ellipse {
    fn render(&self) -> ShapeNode {
        ShapeNode::Ellipse {}
    }
}
