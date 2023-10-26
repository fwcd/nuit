use crate::{View, Node, Bind, Context, Identified};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HStack<T> {
    wrapped: T,
}

impl<T> HStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> Bind for HStack<T> where T: Bind {}

impl<T> View for HStack<T> where T: View {
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::HStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) })
    }
}
