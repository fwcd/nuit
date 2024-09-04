use crate::{View, Node, Bind, Context, IdPath, Event, Id, IdentifyExt};

/// A view that lays out its children on top of each other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlay<T, O> {
    wrapped: T,
    overlayed: O,
}

impl<T, O> Overlay<T, O> {
    pub fn new(wrapped: T, overlayed: O) -> Self {
        Self {
            wrapped,
            overlayed,
        }
    }
}

impl<T, O> Bind for Overlay<T, O> where T: Bind, O: Bind {}

impl<T, O> View for Overlay<T, O> where T: View, O: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                Id::Index(1) => self.overlayed.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on Overlay which only has two childs", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Overlay {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
            overlayed: Box::new(self.overlayed.render(&context.child(1)).identify(1)),
        }
    }
}
