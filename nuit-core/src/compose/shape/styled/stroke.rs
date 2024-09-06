use crate::{Shape, ShapeNode, Style};

/// A shape that strokes its outline using a given style.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke<T> {
    wrapped: T,
    style: Style,
}

impl<T> Stroke<T> {
    pub fn new(wrapped: T, style: Style) -> Self {
        Self {
            wrapped,
            style,
        }
    }
}

impl<T> Shape for Stroke<T> where T: Shape {
    fn render(&self) -> ShapeNode {
        ShapeNode::Stroke {
            wrapped: Box::new(self.wrapped.render()),
            style: self.style.clone(),
        }
    }
}
