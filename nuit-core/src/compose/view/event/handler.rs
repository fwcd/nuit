use nuit_derive::Bind;

use crate::{View, Node, Context, Event, IdPath};

/// A view that handles events using a provided closure.
#[derive(Debug, Clone, Bind, PartialEq, Eq)]
pub struct Handler<T, F> {
    wrapped: T,
    handle_event: F
}

impl<T, F> Handler<T, F> {
    pub fn new(wrapped: T, handle_event: F) -> Self {
        Self {
            wrapped,
            handle_event,
        }
    }
}

impl<T, F> View for Handler<T, F> where T: View, F: Fn(Event) {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if event_path.is_root() {
            (self.handle_event)(event.clone());
        }
        self.wrapped.fire(event, event_path, context);
    }

    fn render(&self, context: &Context) -> Node {
        self.wrapped.render(&context)
    }
}
