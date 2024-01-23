use crate::{View, Node, Bind, Context, Identified, IdPath, Event, Id};

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
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on ZStack which only has one child", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::ZStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) })
    }
}
