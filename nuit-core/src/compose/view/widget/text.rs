use nuit_derive::Bind;

use crate::{View, Node, Context, Event, IdPath};

/// A text label.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
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

impl View for Text {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&self, _context: &Context) -> Node {
        Node::Text { content: self.content.clone() }
    }
}
