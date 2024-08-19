use crate::{View, Node, Bind, Context, Event, IdPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into()
        }
    }
}

impl Bind for Text {}

impl View for Text {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, _context: &Context) -> Node {
        Node::Text { content: self.content.clone() }
    }
}
