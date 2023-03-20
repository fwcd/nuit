use crate::{View, Primitive, Bind, Context, Id};

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
    fn render(&mut self, context: &Context) -> Id<Primitive> {
        context.identify(Primitive::Text { content: self.content.clone() })
    }
}
