use crate::{Shape, ShapeNode, Style};

#[derive(Debug, Clone, PartialEq)]
pub struct Fill<T> {
    wrapped: T,
    style: Style,
}

impl<T> Fill<T> {
    pub fn new(wrapped: T, style: Style) -> Self {
        Self {
            wrapped,
            style,
        }
    }
}

impl<T> Shape for Fill<T> where T: Shape {
    fn render(&self) -> ShapeNode {
        ShapeNode::Fill {
            wrapped: Box::new(self.wrapped.render()),
            style: self.style,
        }
    }
}
