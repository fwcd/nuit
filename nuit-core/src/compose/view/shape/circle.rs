use crate::{Bind, Context, Event, IdPath, Node, ShapeNode, View};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle;

impl Circle {
    pub fn new() -> Self {
        Self
    }
}

impl Bind for Circle {}

impl View for Circle {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, _context: &Context) -> Node {
        Node::Shape {
            shape: ShapeNode::Circle {},
        }
    }
}
