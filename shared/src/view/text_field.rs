use crate::{View, Primitive, Bind, Context, Id, Binding};

#[derive(Debug, Clone)]
pub struct TextField {
    content: Binding<String>,
}

impl TextField {
    pub fn new(content: Binding<String>) -> Self {
        Self { content }
    }
}

impl Bind for TextField {}

impl View for TextField {
    fn render(&mut self, context: &Context) -> Id<Primitive> {
        context.identify(Primitive::TextField { content: self.content.get() })
    }
}
