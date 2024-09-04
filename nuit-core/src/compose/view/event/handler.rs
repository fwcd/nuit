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
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if id_path.is_root() {
            (self.handle_event)(event.clone());
        }
        self.wrapped.fire(event, id_path);
    }

    fn render(&self, context: &Context) -> Node {
        self.wrapped.render(&context)
    }
}
