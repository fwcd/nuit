use crate::{View, Node, Bind, Context, Identified, Event, IdPath, Id};

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
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on Handler which only has one child", i)
            }
        } else {
            (self.handle_event)(event.clone());
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        self.wrapped.render(&context.child(0))
    }
}
