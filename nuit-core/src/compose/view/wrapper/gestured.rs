use nuit_derive::Bind;

use crate::{Context, Event, Gesture, Id, IdPath, IdentifyExt, Node, View};

/// A view recognizing a gesture.
#[derive(Debug, Clone, PartialEq, Bind)]
pub struct Gestured<T, G> {
    wrapped: T,
    gesture: G,
}

impl<T, G> Gestured<T, G> {
    pub const fn new(wrapped: T, gesture: G) -> Self {
        Self { wrapped, gesture }
    }
}

impl<T, G> View for Gestured<T, G> where T: View, G: Gesture {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                Id::Index(1) => match event {
                    Event::Gesture { gesture } => self.gesture.fire(gesture, event_path.tail(), &context.child(1)),
                    _ => eprintln!("Warning: Non-gesture event {event:?} targeted to id path {event_path:?} in a gesture is ignored"),
                },
                i => panic!("Cannot fire event for child id {i} on Gestured, which has two childs"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Gestured {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
            gesture: self.gesture.render(&context.child(1)).identify(1),
        }
    }
}
