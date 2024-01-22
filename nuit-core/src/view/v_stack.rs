use crate::{View, Node, Bind, Context, Identified};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VStack<T> {
    wrapped: T,
}

impl<T> VStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> Bind for VStack<T> where T: Bind {}

impl<T> View for VStack<T> where T: View {
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::VStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) })
    }
}
