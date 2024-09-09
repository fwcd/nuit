use nuit_derive::Bind;

use crate::{Context, Event, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct NavigationDestination<T, D> {
    wrapped: T,
    destination: D,
}

impl<T, D> NavigationDestination<T, D> {
    #[must_use]
    pub const fn new(wrapped: T, destination: D) -> Self {
        Self { wrapped, destination }
    }
}

impl<T, D> View for NavigationDestination<T, D> where T: View, D: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                Id::Index(1) => self.destination.fire(event, event_path.tail(), &context.child(1)),
                i => panic!("Cannot fire event for child id {i} on NavigationDestination which only has two childs"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationDestination {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
            destination: Box::new(self.wrapped.render(&context.child(1)).identify(1)),
        }
    }
}
