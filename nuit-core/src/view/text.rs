use crate::{View, Node, Bind, Context, Identified};

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
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Text { content: self.content.clone() })
    }
}
