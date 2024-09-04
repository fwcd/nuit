use nuit_derive::Bind;

use crate::{View, Node, Context, IdPath, Event, Id, IdentifyExt};

/// A view that lays out its children on top of each other.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
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

impl<T> View for ZStack<T> where T: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on ZStack which only has one child", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::ZStack { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
