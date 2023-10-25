use crate::{View, Node, Bind, Context, Id};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZStack<T> {
    wrapped: T,
}

impl<T> ZStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> Bind for ZStack<T> where T: Bind {}

impl<T> View for ZStack<T> where T: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::ZStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) })
    }
}
