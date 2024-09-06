use nuit_derive::Bind;

use crate::{Context, Event, Id, IdPath, IdentifyExt, ModifierNode, Node, View};

/// A view that applies a modifier.
#[derive(Debug, Clone, PartialEq, Bind)]
pub struct Modified<T> {
    wrapped: T,
    modifier: ModifierNode,
}

impl<T> Modified<T> {
    pub fn new(wrapped: T, modifier: ModifierNode) -> Self {
        Self {
            wrapped,
            modifier,
        }
    }
}

impl<T> View for Modified<T> where T: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {} on Modified which only has one child", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Modified {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
            modifier: self.modifier.clone(),
        }
    }
}
