use nuit_derive::Bind;

use crate::{Context, Event, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct NavigationStack<T> {
    wrapped: T,
}

impl<T> NavigationStack<T> {
    pub const fn new(wrapped: T) -> Self {
        Self {
            wrapped,
        }
    }
}

impl<T> From<T> for NavigationStack<T> {
    fn from(wrapped: T) -> Self {
        Self::new(wrapped)
    }
}

impl<T> View for NavigationStack<T> where T: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on NavigationStack which only has one child"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationStack { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
