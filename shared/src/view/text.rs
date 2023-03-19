use crate::{View, Primitive};

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

impl View for Text {
    fn primitive(&self) -> Primitive {
        Primitive::Text { content: self.content.clone() }
    }
}
