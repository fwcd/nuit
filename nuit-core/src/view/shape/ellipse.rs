use crate::{View, Node, Bind, Context, Event, IdPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ellipse;

impl Ellipse {
    pub fn new() -> Self {
        Self
    }
}

impl Bind for Ellipse {}

impl View for Ellipse {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, _context: &Context) -> Node {
        Node::Ellipse {}
    }
}
