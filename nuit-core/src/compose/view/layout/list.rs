use crate::{Bind, View, Node, Context, IdPath, Event, Id, IdentifyExt};

/// A view that arranges its children in a stylized list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    wrapped: T,
}

impl<T> List<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> Bind for List<T> where T: Bind {}

impl<T> View for List<T> where T: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on List which only has one child", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::List { wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)) }
    }
}
