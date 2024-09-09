use crate::{Bind, Context, Event, EventResponse, IdPath, Node, ShapeNode, View};

/// A composable shape component.
pub trait Shape {
    type Body: Shape = NeverShape;

    fn body(&self) -> Self::Body {
        panic!("Shape does not have a body!")
    }

    fn render(&self) -> ShapeNode {
        Shape::render(&self.body())
    }
}

/// A shape type that can never be constructed.
pub enum NeverShape {}

impl Shape for NeverShape {}

impl<T> Bind for T where T: Shape {}

impl<T> View for T where T: Shape {
    fn fire(&self, _event: &Event, _id_path: &IdPath, _context: &Context) -> EventResponse {
        EventResponse::default()
    }

    fn render(&self, _context: &Context) -> Node {
        Node::Shape {
            shape: Shape::render(self)
        }
    }
}
