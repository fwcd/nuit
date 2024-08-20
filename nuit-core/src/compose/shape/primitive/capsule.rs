use crate::{Shape, ShapeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule;

impl Capsule {
    pub fn new() -> Self {
        Self
    }
}

impl Shape for Capsule {
    fn render(&self) -> ShapeNode {
        ShapeNode::Capsule {}
    }
}
