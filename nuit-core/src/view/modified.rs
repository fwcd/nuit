use crate::{View, Node, Bind, Context, Identified, modifier::Modifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Modified<T> {
    wrapped: T,
    modifier: Modifier,
}

impl<T> Modified<T> {
    pub fn new(wrapped: T, modifier: Modifier) -> Self {
        Self {
            wrapped,
            modifier,
        }
    }
}

impl<T> Bind for Modified<T> where T: Bind {}

impl<T> View for Modified<T> where T: View {
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Modified {
            wrapped: Box::new(self.wrapped.render(&context.child(0))),
            modifier: self.modifier,
        })
    }
}
