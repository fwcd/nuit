use crate::{Bind, View, Node, Context, Identified, Id};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    wrapped: T,
}

impl<T> List<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> Bind for List<T> where T: Bind {}

impl<T> View for List<T> where T: View {
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::List { wrapped: Box::new(self.wrapped.render(&context.child(Id::index(0)))) })
    }
}
