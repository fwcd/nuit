use crate::{View, Node, Bind, Context, Event, IdPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule;

impl Capsule {
    pub fn new() -> Self {
        Self
    }
}

impl Bind for Capsule {}

impl View for Capsule {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, _context: &Context) -> Node {
        Node::Capsule {}
    }
}
