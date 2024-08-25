use crate::{View, Node, Bind, Context, IdPath, Event, Id, IdentifyExt};

/// A view that lays out its children vertically.
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
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on VStack which only has one child", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::VStack { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
