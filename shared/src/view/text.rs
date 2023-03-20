use crate::{View, Primitive, Bind, Storage};

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
    fn render(&self, _storage: &Storage) -> Primitive {
        Primitive::Text { content: self.content.clone() }
    }
}
