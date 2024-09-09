use nuit_derive::Bind;
use serde::Serialize;

use crate::{Context, Event, Id, IdPath, IdentifyExt, Node, Text, View};

#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct NavigationLink<L, V> {
    label: L,
    value: V,
}

impl<L, V> NavigationLink<L, V> {
    #[must_use]
    pub const fn new(label: L, value: V) -> Self {
        Self { label, value }
    }
}

impl<V> NavigationLink<Text, V> {
    #[must_use]
    pub fn with_text(label: impl Into<String>, value: V) -> Self {
        Self {
            label: Text::new(label),
            value,
        }
    }
}

impl<L, V> View for NavigationLink<L, V> where L: View, V: Serialize {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.label.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on NavigationLink which only has one child"),
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationLink {
            label: Box::new(self.label.render(&context.child(0)).identify(0)),
            value: serde_json::to_value(&self.value).expect("Could not serialize navigation link value"),
        }
    }
}
