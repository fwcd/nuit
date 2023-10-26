use crate::{View, Node, Bind, Context, Id, Insets};

#[derive(Debug, Clone, PartialEq)]
pub struct Padding<T> {
    wrapped: T,
    insets: Insets,
}

impl<T> Padding<T> {
    pub fn new(wrapped: T, insets: Insets) -> Self {
        Self {
            wrapped,
            insets,
        }
    }
}

impl<T> Bind for Padding<T> where T: Bind {}

impl<T> View for Padding<T> where T: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Padding {
            wrapped: Box::new(self.wrapped.render(&context.child(0))),
            insets: self.insets,
        })
    }
}
