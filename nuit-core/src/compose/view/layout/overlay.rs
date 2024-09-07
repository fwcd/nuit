use nuit_derive::Bind;

use crate::{Alignment, Context, Event, Id, IdPath, IdentifyExt, Node, View};

/// A view that lays out its children on top of each other.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct Overlay<T, O> {
    wrapped: T,
    alignment: Alignment,
    overlayed: O,
}

impl<T, O> Overlay<T, O> {
    pub const fn new(wrapped: T, alignment: Alignment, overlayed: O) -> Self {
        Self {
            wrapped,
            alignment,
            overlayed,
        }
    }
}

impl<T, O> View for Overlay<T, O> where T: View, O: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                Id::Index(1) => self.overlayed.fire(event, event_path.tail(), &context.child(1)),
                i => panic!("Cannot fire event for child id {i} on Overlay which only has two childs"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Overlay {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
            alignment: self.alignment,
            overlayed: Box::new(self.overlayed.render(&context.child(1)).identify(1)),
        }
    }
}
