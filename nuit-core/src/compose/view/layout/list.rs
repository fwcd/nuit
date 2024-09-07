use nuit_derive::Bind;

use crate::{View, Node, Context, IdPath, Event, Id, IdentifyExt};

/// A view that arranges its children in a stylized list.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct List<T> {
    wrapped: T,
}

impl<T> List<T> {
    pub const fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> View for List<T> where T: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on List which only has one child"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::List { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
