use crate::{View, Node, Bind, Context, Event, IdPath};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl<T, F> Bind for Handler<T, F> where T: Bind, F: Fn(Event) {}

impl<T, F> View for Handler<T, F> where T: View, F: Fn(Event) {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if id_path.is_root() {
            (self.handle_event)(event.clone());
        } else {
            self.wrapped.fire(event, id_path);
        }
    }

    fn render(&mut self, context: &Context) -> Node {
        self.wrapped.render(&context)
    }
}
