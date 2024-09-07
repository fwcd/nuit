use crate::{Shape, ShapeNode};

/// A capsule shape.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Capsule;

impl Capsule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Shape for Capsule {
    fn render(&self) -> ShapeNode {
        ShapeNode::Capsule {}
    }
}
