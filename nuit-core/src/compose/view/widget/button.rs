use nuit_derive::Bind;

use crate::{View, Node, Context, Event, IdPath, Id, IdentifyExt};

use super::Text;

/// A widget that performs an action when pressed/tapped.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct Button<T, F> {
    label: T,
    action: Option<F>,
}

impl<T, F> Button<T, F> {
    #[must_use]
    pub fn new(label: T, action: F) -> Self {
        Self {
            label,
            action: Some(action),
        }
    }
}

impl<F> Button<Text, F> {
    #[must_use]
    pub fn with_text(label: impl Into<String>, action: F) -> Self {
        Self {
            label: Text::new(label),
            action: Some(action),
        }
    }
}

impl<T, F> View for Button<T, F> where T: View, F: Fn() + 'static {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.label.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on Button which only has one child"),
            }
        } else if let Event::ButtonTap {} = event {
            if let Some(ref action) = self.action {
                action();
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Button { label: Box::new(self.label.render(&context.child(0)).identify(0)) }
    }
}
