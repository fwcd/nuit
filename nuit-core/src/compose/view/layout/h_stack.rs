use crate::{View, Node, Bind, Context, Event, IdPath, Id, IdentifyExt};

/// A view that lays out its children horizontally.
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
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on HStack which only has one child", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Node {
        Node::HStack { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
